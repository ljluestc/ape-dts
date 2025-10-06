#[cfg(feature = "metrics")]
use std::collections::HashMap;
use std::{
    fs::{self, File},
    io::Read,
};

use anyhow::{bail, Ok};

#[cfg(feature = "metrics")]
use crate::config::metrics_config::MetricsConfig;
use crate::error::Error;

use super::{
    config_enums::{
        ConflictPolicyEnum, DbType, ExtractType, MetaCenterType, ParallelType, PipelineType,
        SinkType,
    },
    data_marker_config::DataMarkerConfig,
    extractor_config::{BasicExtractorConfig, ExtractorConfig},
    filter_config::FilterConfig,
    ini_loader::IniLoader,
    meta_center_config::MetaCenterConfig,
    parallelizer_config::ParallelizerConfig,
    pipeline_config::PipelineConfig,
    processor_config::ProcessorConfig,
    resumer_config::ResumerConfig,
    router_config::RouterConfig,
    runtime_config::RuntimeConfig,
    s3_config::S3Config,
    sinker_config::{BasicSinkerConfig, SinkerConfig},
};

#[derive(Clone)]
pub struct TaskConfig {
    pub extractor_basic: BasicExtractorConfig,
    pub extractor: ExtractorConfig,
    pub sinker_basic: BasicSinkerConfig,
    pub sinker: SinkerConfig,
    pub runtime: RuntimeConfig,
    pub parallelizer: ParallelizerConfig,
    pub pipeline: PipelineConfig,
    pub filter: FilterConfig,
    pub router: RouterConfig,
    pub resumer: ResumerConfig,
    pub meta_center: Option<MetaCenterConfig>,
    pub data_marker: Option<DataMarkerConfig>,
    pub processor: Option<ProcessorConfig>,
    #[cfg(feature = "metrics")]
    pub metrics: MetricsConfig,
}

// sections
const EXTRACTOR: &str = "extractor";
const SINKER: &str = "sinker";
const PIPELINE: &str = "pipeline";
const PARALLELIZER: &str = "parallelizer";
const RUNTIME: &str = "runtime";
const FILTER: &str = "filter";
const ROUTER: &str = "router";
const RESUMER: &str = "resumer";
const DATA_MARKER: &str = "data_marker";
const PROCESSOR: &str = "processor";
const META_CENTER: &str = "metacenter";
// keys
const CHECK_LOG_DIR: &str = "check_log_dir";
const DB_TYPE: &str = "db_type";
const URL: &str = "url";
const BATCH_SIZE: &str = "batch_size";
const SAMPLE_INTERVAL: &str = "sample_interval";
const HEARTBEAT_INTERVAL_SECS: &str = "heartbeat_interval_secs";
const KEEPALIVE_INTERVAL_SECS: &str = "keepalive_interval_secs";
const HEARTBEAT_TB: &str = "heartbeat_tb";
const APP_NAME: &str = "app_name";
const REVERSE: &str = "reverse";
const REPL_PORT: &str = "repl_port";
const PARALLEL_SIZE: &str = "parallel_size";
const DDL_CONFLICT_POLICY: &str = "ddl_conflict_policy";
const REPLACE: &str = "replace";
const DISABLE_FOREIGN_KEY_CHECKS: &str = "disable_foreign_key_checks";
// default values
const APE_DTS: &str = "APE_DTS";
const ASTRISK: &str = "*";

impl TaskConfig {
    pub fn new(task_config_file: &str) -> anyhow::Result<Self> {
        let loader = IniLoader::new(task_config_file);

        let pipeline = Self::load_pipeline_config(&loader);
        let runtime = Self::load_runtime_config(&loader)?;
        let resumer = Self::load_resumer_config(&loader, &runtime)?;
        let (extractor_basic, extractor) = Self::load_extractor_config(&loader, &pipeline)?;
        let (sinker_basic, sinker) = Self::load_sinker_config(&loader)?;
        Ok(Self {
            extractor_basic,
            extractor,
            parallelizer: Self::load_parallelizer_config(&loader)?,
            pipeline,
            sinker_basic,
            sinker,
            runtime,
            filter: Self::load_filter_config(&loader)?,
            router: Self::load_router_config(&loader)?,
            resumer,
            data_marker: Self::load_data_marker_config(&loader)?,
            processor: Self::load_processor_config(&loader)?,
            meta_center: Self::load_meta_center_config(&loader)?,
            #[cfg(feature = "metrics")]
            metrics: Self::load_metrics_config(&loader)?,
        })
    }

    fn load_extractor_config(
        loader: &IniLoader,
        pipeline: &PipelineConfig,
    ) -> anyhow::Result<(BasicExtractorConfig, ExtractorConfig)> {
        let db_type: DbType = loader.get_required(EXTRACTOR, DB_TYPE);
        let extract_type: ExtractType = loader.get_required(EXTRACTOR, "extract_type");
        let url: String = loader.get_optional(EXTRACTOR, URL);
        let heartbeat_interval_secs: u64 =
            loader.get_with_default(EXTRACTOR, HEARTBEAT_INTERVAL_SECS, 10);
        let keepalive_interval_secs: u64 =
            loader.get_with_default(EXTRACTOR, KEEPALIVE_INTERVAL_SECS, 10);
        let heartbeat_tb = loader.get_optional(EXTRACTOR, HEARTBEAT_TB);
        let batch_size = loader.get_with_default(EXTRACTOR, BATCH_SIZE, pipeline.buffer_size);

        let basic = BasicExtractorConfig {
            db_type: db_type.clone(),
            extract_type: extract_type.clone(),
            url: url.clone(),
        };

        let not_supported_err =
            Error::ConfigError(format!("extract type: {} not supported", extract_type));

        let extractor = match db_type {
            DbType::Mysql => match extract_type {
                ExtractType::Snapshot => ExtractorConfig::MysqlSnapshot {
                    url,
                    db: String::new(),
                    tb: String::new(),
                    sample_interval: loader.get_with_default(EXTRACTOR, SAMPLE_INTERVAL, 1),
                    parallel_size: loader.get_with_default(EXTRACTOR, PARALLEL_SIZE, 1),
                    batch_size,
                },

                ExtractType::Cdc => ExtractorConfig::MysqlCdc {
                    url,
                    binlog_filename: loader.get_optional(EXTRACTOR, "binlog_filename"),
                    binlog_position: loader.get_optional(EXTRACTOR, "binlog_position"),
                    server_id: loader.get_required(EXTRACTOR, "server_id"),
                    gtid_enabled: loader.get_optional(EXTRACTOR, "gtid_enabled"),
                    gtid_set: loader.get_optional(EXTRACTOR, "gtid_set"),
                    binlog_heartbeat_interval_secs: loader.get_with_default(
                        EXTRACTOR,
                        "binlog_heartbeat_interval_secs",
                        10,
                    ),
                    binlog_timeout_secs: loader.get_with_default(
                        EXTRACTOR,
                        "binlog_timeout_secs",
                        60,
                    ),
                    heartbeat_interval_secs,
                    heartbeat_tb,
                    start_time_utc: loader.get_optional(EXTRACTOR, "start_time_utc"),
                    end_time_utc: loader.get_optional(EXTRACTOR, "end_time_utc"),
                },

                ExtractType::CheckLog => ExtractorConfig::MysqlCheck {
                    url,
                    check_log_dir: loader.get_required(EXTRACTOR, CHECK_LOG_DIR),
                    batch_size: loader.get_with_default(EXTRACTOR, BATCH_SIZE, 200),
                },

                ExtractType::Struct => ExtractorConfig::MysqlStruct {
                    url,
                    db: String::new(),
                },

                ExtractType::FoxlakeS3 => {
                    let s3_config = S3Config {
                        bucket: loader.get_optional(EXTRACTOR, "s3_bucket"),
                        access_key: loader.get_optional(EXTRACTOR, "s3_access_key"),
                        secret_key: loader.get_optional(EXTRACTOR, "s3_secret_key"),
                        region: loader.get_optional(EXTRACTOR, "s3_region"),
                        endpoint: loader.get_optional(EXTRACTOR, "s3_endpoint"),
                        root_dir: loader.get_optional(EXTRACTOR, "s3_root_dir"),
                        root_url: loader.get_optional(EXTRACTOR, "s3_root_url"),
                    };
                    ExtractorConfig::FoxlakeS3 {
                        url,
                        schema: String::new(),
                        tb: String::new(),
                        s3_config,
                        batch_size,
                    }
                }

                _ => bail! {not_supported_err},
            },

            DbType::Pg => match extract_type {
                ExtractType::Snapshot => ExtractorConfig::PgSnapshot {
                    url,
                    schema: String::new(),
                    tb: String::new(),
                    sample_interval: loader.get_with_default(EXTRACTOR, SAMPLE_INTERVAL, 1),
                    batch_size,
                },

                ExtractType::Cdc => ExtractorConfig::PgCdc {
                    url,
                    slot_name: loader.get_required(EXTRACTOR, "slot_name"),
                    pub_name: loader.get_optional(EXTRACTOR, "pub_name"),
                    start_lsn: loader.get_optional(EXTRACTOR, "start_lsn"),
                    recreate_slot_if_exists: loader
                        .get_optional(EXTRACTOR, "recreate_slot_if_exists"),
                    keepalive_interval_secs,
                    heartbeat_interval_secs,
                    heartbeat_tb,
                    ddl_meta_tb: loader.get_optional(EXTRACTOR, "ddl_meta_tb"),
                    start_time_utc: loader.get_optional(EXTRACTOR, "start_time_utc"),
                    end_time_utc: loader.get_optional(EXTRACTOR, "end_time_utc"),
                },

                ExtractType::CheckLog => ExtractorConfig::PgCheck {
                    url,
                    check_log_dir: loader.get_required(EXTRACTOR, CHECK_LOG_DIR),
                    batch_size: loader.get_with_default(EXTRACTOR, BATCH_SIZE, 200),
                },

                ExtractType::Struct => ExtractorConfig::PgStruct {
                    url,
                    schema: String::new(),
                    do_global_structs: false,
                },

                _ => bail! { not_supported_err },
            },

            DbType::Mongo => {
                let app_name: String =
                    loader.get_with_default(EXTRACTOR, APP_NAME, APE_DTS.to_string());
                match extract_type {
                    ExtractType::Snapshot => ExtractorConfig::MongoSnapshot {
                        url,
                        app_name,
                        db: String::new(),
                        tb: String::new(),
                    },

                    ExtractType::Cdc => ExtractorConfig::MongoCdc {
                        url,
                        app_name,
                        resume_token: loader.get_optional(EXTRACTOR, "resume_token"),
                        start_timestamp: loader.get_optional(EXTRACTOR, "start_timestamp"),
                        source: loader.get_optional(EXTRACTOR, "source"),
                        heartbeat_interval_secs,
                        heartbeat_tb,
                    },

                    ExtractType::CheckLog => ExtractorConfig::MongoCheck {
                        url,
                        app_name,
                        check_log_dir: loader.get_required(EXTRACTOR, CHECK_LOG_DIR),
                        batch_size: loader.get_with_default(EXTRACTOR, BATCH_SIZE, 200),
                    },

                    _ => bail! { not_supported_err },
                }
            }

            DbType::Redis => match extract_type {
                ExtractType::Snapshot => {
                    let repl_port = loader.get_with_default(EXTRACTOR, REPL_PORT, 10008);
                    ExtractorConfig::RedisSnapshot { url, repl_port }
                }

                ExtractType::SnapshotFile => ExtractorConfig::RedisSnapshotFile {
                    file_path: loader.get_required(EXTRACTOR, "file_path"),
                },

                ExtractType::Scan => ExtractorConfig::RedisScan {
                    url,
                    statistic_type: loader.get_required(EXTRACTOR, "statistic_type"),
                    scan_count: loader.get_with_default(EXTRACTOR, "scan_count", 1000),
                },

                ExtractType::Cdc => {
                    let repl_port = loader.get_with_default(EXTRACTOR, REPL_PORT, 10008);
                    ExtractorConfig::RedisCdc {
                        url,
                        repl_port,
                        repl_id: loader.get_optional(EXTRACTOR, "repl_id"),
                        repl_offset: loader.get_optional(EXTRACTOR, "repl_offset"),
                        keepalive_interval_secs,
                        heartbeat_interval_secs,
                        heartbeat_key: loader.get_optional(EXTRACTOR, "heartbeat_key"),
                        now_db_id: loader.get_optional(EXTRACTOR, "now_db_id"),
                    }
                }

                ExtractType::SnapshotAndCdc => {
                    let repl_port = loader.get_with_default(EXTRACTOR, REPL_PORT, 10008);
                    ExtractorConfig::RedisSnapshotAndCdc {
                        url,
                        repl_port,
                        repl_id: loader.get_optional(EXTRACTOR, "repl_id"),
                        keepalive_interval_secs,
                        heartbeat_interval_secs,
                        heartbeat_key: loader.get_optional(EXTRACTOR, "heartbeat_key"),
                    }
                }

                ExtractType::Reshard => ExtractorConfig::RedisReshard { url },

                _ => bail! { not_supported_err },
            },

            DbType::Kafka => ExtractorConfig::Kafka {
                url,
                group: loader.get_required(EXTRACTOR, "group"),
                topic: loader.get_required(EXTRACTOR, "topic"),
                partition: loader.get_optional(EXTRACTOR, "partition"),
                offset: loader.get_optional(EXTRACTOR, "offset"),
                ack_interval_secs: loader.get_optional(EXTRACTOR, "ack_interval_secs"),
            },

            db_type => {
                bail! {Error::ConfigError(format!(
                    "extractor db type: {} not supported",
                    db_type
                ))}
            }
        };
        Ok((basic, extractor))
    }

    fn load_sinker_config(loader: &IniLoader) -> anyhow::Result<(BasicSinkerConfig, SinkerConfig)> {
        let sink_type = loader.get_with_default(SINKER, "sink_type", SinkType::Write);
        if let SinkType::Dummy = sink_type {
            return Ok((BasicSinkerConfig::default(), SinkerConfig::Dummy));
        }

        let db_type: DbType = loader.get_required(SINKER, DB_TYPE);
        let url: String = loader.get_optional(SINKER, URL);
        let batch_size: usize = loader.get_with_default(SINKER, BATCH_SIZE, 200);

        let basic = BasicSinkerConfig {
            sink_type: sink_type.clone(),
            db_type: db_type.clone(),
            url: url.clone(),
            batch_size,
        };

        let conflict_policy: ConflictPolicyEnum =
            loader.get_with_default(SINKER, "conflict_policy", ConflictPolicyEnum::Interrupt);

        let not_supported_err =
            Error::ConfigError(format!("sinker db type: {} not supported", db_type));

        let sinker = match db_type {
            DbType::Mysql | DbType::Tidb => match sink_type {
                SinkType::Write => SinkerConfig::Mysql {
                    url,
                    batch_size,
                    replace: loader.get_with_default(SINKER, REPLACE, true),
                    disable_foreign_key_checks: loader.get_with_default(
                        SINKER,
                        DISABLE_FOREIGN_KEY_CHECKS,
                        true,
                    ),
                },

                SinkType::Check => SinkerConfig::MysqlCheck {
                    url,
                    batch_size,
                    check_log_dir: loader.get_optional(SINKER, CHECK_LOG_DIR),
                },

                SinkType::Struct => SinkerConfig::MysqlStruct {
                    url,
                    conflict_policy,
                },

                SinkType::Sql => SinkerConfig::Sql {
                    reverse: loader.get_optional(SINKER, REVERSE),
                },

                _ => bail! { not_supported_err },
            },

            DbType::Pg => match sink_type {
                SinkType::Write => SinkerConfig::Pg {
                    url,
                    batch_size,
                    replace: loader.get_with_default(SINKER, REPLACE, true),
                    disable_foreign_key_checks: loader.get_with_default(
                        SINKER,
                        DISABLE_FOREIGN_KEY_CHECKS,
                        true,
                    ),
                },

                SinkType::Check => SinkerConfig::PgCheck {
                    url,
                    batch_size,
                    check_log_dir: loader.get_optional(SINKER, CHECK_LOG_DIR),
                },

                SinkType::Struct => SinkerConfig::PgStruct {
                    url,
                    conflict_policy,
                },

                SinkType::Sql => SinkerConfig::Sql {
                    reverse: loader.get_optional(SINKER, REVERSE),
                },

                _ => bail! { not_supported_err },
            },

            DbType::Mongo => {
                let app_name: String =
                    loader.get_with_default(SINKER, APP_NAME, APE_DTS.to_string());
                match sink_type {
                    SinkType::Write => SinkerConfig::Mongo {
                        url,
                        app_name,
                        batch_size,
                    },

                    SinkType::Check => SinkerConfig::MongoCheck {
                        url,
                        app_name,
                        batch_size,
                        check_log_dir: loader.get_optional(SINKER, CHECK_LOG_DIR),
                    },

                    _ => bail! { not_supported_err },
                }
            }

            DbType::Kafka => SinkerConfig::Kafka {
                url,
                batch_size,
                ack_timeout_secs: loader.get_with_default(SINKER, "ack_timeout_secs", 5),
                required_acks: loader.get_with_default(SINKER, "required_acks", "one".to_string()),
                with_field_defs: loader.get_with_default(SINKER, "with_field_defs", true),
            },

            DbType::Redis => match sink_type {
                SinkType::Write => SinkerConfig::Redis {
                    url,
                    batch_size,
                    method: loader.get_optional(SINKER, "method"),
                    is_cluster: loader.get_optional(SINKER, "is_cluster"),
                },

                SinkType::Statistic => SinkerConfig::RedisStatistic {
                    statistic_type: loader.get_required(SINKER, "statistic_type"),
                    data_size_threshold: loader.get_optional(SINKER, "data_size_threshold"),
                    freq_threshold: loader.get_optional(SINKER, "freq_threshold"),
                    statistic_log_dir: loader.get_optional(SINKER, "statistic_log_dir"),
                },

                _ => bail! { not_supported_err },
            },

            DbType::StarRocks => match sink_type {
                SinkType::Write => SinkerConfig::StarRocks {
                    url,
                    batch_size,
                    stream_load_url: loader.get_optional(SINKER, "stream_load_url"),
                    hard_delete: loader.get_optional(SINKER, "hard_delete"),
                },

                SinkType::Struct => SinkerConfig::StarRocksStruct {
                    url,
                    conflict_policy,
                },

                _ => bail! { not_supported_err },
            },

            DbType::Doris => match sink_type {
                SinkType::Write => SinkerConfig::Doris {
                    url,
                    batch_size,
                    stream_load_url: loader.get_optional(SINKER, "stream_load_url"),
                },

                SinkType::Struct => SinkerConfig::DorisStruct {
                    url,
                    conflict_policy,
                },

                _ => bail! { not_supported_err },
            },

            DbType::ClickHouse => match sink_type {
                SinkType::Write => SinkerConfig::ClickHouse { url, batch_size },

                SinkType::Struct => SinkerConfig::ClickhouseStruct {
                    url,
                    conflict_policy,
                    engine: loader.get_with_default(
                        SINKER,
                        "engine",
                        "ReplacingMergeTree".to_string(),
                    ),
                },

                _ => bail! { not_supported_err },
            },

            DbType::Foxlake => {
                let s3_config = S3Config {
                    bucket: loader.get_optional(SINKER, "s3_bucket"),
                    access_key: loader.get_optional(SINKER, "s3_access_key"),
                    secret_key: loader.get_optional(SINKER, "s3_secret_key"),
                    region: loader.get_optional(SINKER, "s3_region"),
                    endpoint: loader.get_optional(SINKER, "s3_endpoint"),
                    root_dir: loader.get_optional(SINKER, "s3_root_dir"),
                    root_url: loader.get_optional(SINKER, "s3_root_url"),
                };

                match sink_type {
                    SinkType::Write => SinkerConfig::Foxlake {
                        url,
                        batch_size,
                        batch_memory_mb: loader.get_optional(SINKER, "batch_memory_mb"),
                        s3_config,
                        engine: loader.get_optional(SINKER, "engine"),
                    },

                    SinkType::Struct => SinkerConfig::FoxlakeStruct {
                        url,
                        conflict_policy,
                        engine: loader.get_optional(SINKER, "engine"),
                    },

                    SinkType::Push => SinkerConfig::FoxlakePush {
                        url,
                        batch_size,
                        batch_memory_mb: loader.get_optional(SINKER, "batch_memory_mb"),
                        s3_config,
                    },

                    SinkType::Merge => SinkerConfig::FoxlakeMerge {
                        url,
                        batch_size,
                        s3_config,
                    },

                    _ => bail! { not_supported_err },
                }
            }
        };
        Ok((basic, sinker))
    }

    fn load_parallelizer_config(loader: &IniLoader) -> anyhow::Result<ParallelizerConfig> {
        Ok(ParallelizerConfig {
            parallel_size: loader.get_with_default(PARALLELIZER, PARALLEL_SIZE, 1),
            parallel_type: loader.get_with_default(
                PARALLELIZER,
                "parallel_type",
                ParallelType::Serial,
            ),
        })
    }

    fn load_pipeline_config(loader: &IniLoader) -> PipelineConfig {
        let mut config = PipelineConfig {
            buffer_size: loader.get_with_default(PIPELINE, "buffer_size", 16000),
            checkpoint_interval_secs: loader.get_with_default(
                PIPELINE,
                "checkpoint_interval_secs",
                10,
            ),
            batch_sink_interval_secs: loader.get_optional(PIPELINE, "batch_sink_interval_secs"),
            counter_time_window_secs: loader.get_optional(PIPELINE, "counter_time_window_secs"),
            counter_max_sub_count: loader.get_with_default(PIPELINE, "counter_max_sub_count", 1000),
            max_rps: loader.get_optional(PIPELINE, "max_rps"),
            buffer_memory_mb: loader.get_optional(PIPELINE, "buffer_memory_mb"),
            pipeline_type: loader.get_with_default(PIPELINE, "pipeline_type", PipelineType::Basic),
            http_host: loader.get_with_default(PIPELINE, "http_host", "0.0.0.0".to_string()),
            http_port: loader.get_with_default(PIPELINE, "http_port", 10231),
            with_field_defs: loader.get_with_default(PIPELINE, "with_field_defs", true),
        };

        if config.counter_time_window_secs == 0 {
            config.counter_time_window_secs = config.checkpoint_interval_secs;
        }
        config
    }

    fn load_runtime_config(loader: &IniLoader) -> anyhow::Result<RuntimeConfig> {
        Ok(RuntimeConfig {
            log_level: loader.get_with_default(RUNTIME, "log_level", "info".to_string()),
            log_dir: loader.get_with_default(RUNTIME, "log_dir", "./logs".to_string()),
            log4rs_file: loader.get_with_default(
                RUNTIME,
                "log4rs_file",
                "./log4rs.yaml".to_string(),
            ),
            tb_parallel_size: loader.get_with_default(RUNTIME, "tb_parallel_size", 1),
        })
    }

    fn load_filter_config(loader: &IniLoader) -> anyhow::Result<FilterConfig> {
        Ok(FilterConfig {
            do_schemas: loader.get_optional(FILTER, "do_dbs"),
            ignore_schemas: loader.get_optional(FILTER, "ignore_dbs"),
            do_schemas_regex: loader.get_optional(FILTER, "do_dbs_regex"),
            ignore_schemas_regex: loader.get_optional(FILTER, "ignore_dbs_regex"),
            do_tbs: loader.get_optional(FILTER, "do_tbs"),
            ignore_tbs: loader.get_optional(FILTER, "ignore_tbs"),
            do_tbs_regex: loader.get_optional(FILTER, "do_tbs_regex"),
            ignore_tbs_regex: loader.get_optional(FILTER, "ignore_tbs_regex"),
            ignore_cols: loader.get_optional(FILTER, "ignore_cols"),
            do_cols: loader.get_optional(FILTER, "do_cols"),
            do_cols_regex: loader.get_optional(FILTER, "do_cols_regex"),
            ignore_cols_regex: loader.get_optional(FILTER, "ignore_cols_regex"),
            do_events: loader.get_optional(FILTER, "do_events"),
            do_ddls: loader.get_optional(FILTER, "do_ddls"),
            do_dcls: loader.get_optional(FILTER, "do_dcls"),
            do_structures: loader.get_with_default(FILTER, "do_structures", ASTRISK.to_string()),
            ignore_cmds: loader.get_optional(FILTER, "ignore_cmds"),
            where_conditions: loader.get_optional(FILTER, "where_conditions"),
            content_filters: loader.get_optional(FILTER, "content_filters"),
        })
    }

    fn load_router_config(loader: &IniLoader) -> anyhow::Result<RouterConfig> {
        Ok(RouterConfig::Rdb {
            schema_map: loader.get_optional(ROUTER, "db_map"),
            tb_map: loader.get_optional(ROUTER, "tb_map"),
            col_map: loader.get_optional(ROUTER, "col_map"),
            topic_map: loader.get_optional(ROUTER, "topic_map"),
            content_routes: loader.get_optional(ROUTER, "content_routes"),
        })
    }

    fn load_resumer_config(
        loader: &IniLoader,
        runtime: &RuntimeConfig,
    ) -> anyhow::Result<ResumerConfig> {
        let resume_log_dir: String =
            loader.get_with_default(RESUMER, "resume_log_dir", runtime.log_dir.clone());
        Ok(ResumerConfig {
            resume_config_file: loader.get_optional(RESUMER, "resume_config_file"),
            resume_from_log: loader.get_optional(RESUMER, "resume_from_log"),
            resume_log_dir,
        })
    }

    fn load_data_marker_config(loader: &IniLoader) -> anyhow::Result<Option<DataMarkerConfig>> {
        if !loader.ini.sections().contains(&DATA_MARKER.to_string()) {
            return Ok(None);
        }

        Ok(Some(DataMarkerConfig {
            topo_name: loader.get_required(DATA_MARKER, "topo_name"),
            topo_nodes: loader.get_optional(DATA_MARKER, "topo_nodes"),
            src_node: loader.get_required(DATA_MARKER, "src_node"),
            dst_node: loader.get_required(DATA_MARKER, "dst_node"),
            do_nodes: loader.get_required(DATA_MARKER, "do_nodes"),
            ignore_nodes: loader.get_optional(DATA_MARKER, "ignore_nodes"),
            marker: loader.get_required(DATA_MARKER, "marker"),
        }))
    }

    fn load_processor_config(loader: &IniLoader) -> anyhow::Result<Option<ProcessorConfig>> {
        if !loader.ini.sections().contains(&PROCESSOR.to_string()) {
            return Ok(None);
        }

        let lua_code_file = loader.get_optional(PROCESSOR, "lua_code_file");
        let mut lua_code = String::new();

        if fs::metadata(&lua_code_file).is_ok() {
            let mut file = File::open(&lua_code_file).expect("failed to open lua code file");
            file.read_to_string(&mut lua_code)
                .expect("failed to read lua code file");
        }

        Ok(Some(ProcessorConfig {
            lua_code_file,
            lua_code,
        }))
    }

    fn load_meta_center_config(loader: &IniLoader) -> anyhow::Result<Option<MetaCenterConfig>> {
        let mut config = MetaCenterConfig::Basic;
        let db_type: DbType = loader.get_required(EXTRACTOR, DB_TYPE);
        let meta_type = loader.get_with_default(META_CENTER, "type", MetaCenterType::Basic);
        if meta_type == MetaCenterType::DbEngine && db_type == DbType::Mysql {
            let extractor_url: String = loader.get_required(EXTRACTOR, URL);
            let sinker_url: String = loader.get_required(SINKER, URL);
            let meta_center_url: String = loader.get_required(META_CENTER, URL);
            if extractor_url == meta_center_url || sinker_url == meta_center_url {
                panic!(
                    "config, [{}].{} should be different with [{}].{} and [{}].{}",
                    META_CENTER, URL, EXTRACTOR, URL, SINKER, URL
                );
            }

            config = MetaCenterConfig::MySqlDbEngine {
                url: meta_center_url,
                ddl_conflict_policy: loader.get_with_default(
                    META_CENTER,
                    DDL_CONFLICT_POLICY,
                    ConflictPolicyEnum::Interrupt,
                ),
            }
        }
        Ok(Some(config))
    }

    #[cfg(feature = "metrics")]
    fn load_metrics_config(loader: &IniLoader) -> anyhow::Result<MetricsConfig> {
        let metrics_section = "metrics";
        let labels_str: String = loader.get_optional(metrics_section, "labels");
        let mut metrics_labels = HashMap::new();
        if !labels_str.is_empty() {
            for label_pair in labels_str.split(',') {
                if let Some((key, value)) = label_pair.trim().split_once('=') {
                    metrics_labels.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }
        Ok(MetricsConfig {
            http_host: loader.get_with_default(metrics_section, "http_host", "0.0.0.0".to_string()),
            http_port: loader.get_with_default(metrics_section, "http_port", 9090),
            workers: loader.get_with_default(metrics_section, "workers", 2),
            metrics_labels,
        })
    }
}

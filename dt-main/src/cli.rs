use clap::{Parser, Subcommand};

pub const DEFAULT_CONFIG_FILE: &str = "config.ini";

/// ape-dts: ApeCloud Data Transfer Service
///
/// A comprehensive data transfer and synchronization tool supporting multiple databases
/// and replication modes including snapshot, CDC, structure migration, and data validation.
#[derive(Parser, Debug, Clone)]
#[command(name = "ape-dts")]
#[command(about = "ApeCloud Data Transfer Service")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(long_about = "A comprehensive data transfer and synchronization tool supporting multiple databases \
and replication modes including snapshot, CDC, structure migration, and data validation.\n\n\
Examples:\n  \
ape-dts snapshot --config my-config.ini\n  \
ape-dts cdc --config streaming.ini --log-level debug\n  \
ape-dts check --config validation.ini --dry-run\n  \
ape-dts struct --config migration.ini --verbose")]
pub struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = DEFAULT_CONFIG_FILE, help = "Path to configuration file")]
    pub config: String,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, value_name = "LEVEL", help = "Set logging level")]
    pub log_level: Option<String>,

    /// Dry run mode - validate configuration without executing
    #[arg(short, long, help = "Validate configuration and show what would be executed without running")]
    pub dry_run: bool,

    /// Enable verbose output
    #[arg(short, long, help = "Enable verbose output with detailed progress information")]
    pub verbose: bool,

    /// Skip configuration validation
    #[arg(long, help = "Skip configuration file validation (use with caution)")]
    pub skip_validation: bool,

    /// Maximum parallel workers (overrides config file setting)
    #[arg(long, value_name = "NUM", help = "Override max parallel workers from config")]
    pub max_parallel: Option<u32>,

    /// Batch size for operations (overrides config file setting)
    #[arg(long, value_name = "SIZE", help = "Override batch size from config")]
    pub batch_size: Option<u32>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Create a snapshot of source data
    ///
    /// Performs a full data snapshot from the source database to the target.
    /// This is typically used for initial data synchronization.
    Snapshot {
        /// Source database URL
        #[arg(short, long)]
        source: Option<String>,

        /// Target database URL
        #[arg(short, long)]
        target: Option<String>,

        /// Batch size for snapshot operations
        #[arg(short, long)]
        batch_size: Option<u32>,

        /// Number of parallel workers
        #[arg(short, long)]
        parallel: Option<u32>,
    },

    /// Start Change Data Capture (CDC) replication
    ///
    /// Continuously replicates changes from source to target database.
    /// Monitors transaction logs for real-time data synchronization.
    Cdc {
        /// Source database URL
        #[arg(short, long)]
        source: Option<String>,

        /// Target database URL
        #[arg(short, long)]
        target: Option<String>,

        /// Starting position for CDC (binlog position, LSN, etc.)
        #[arg(short, long)]
        position: Option<String>,

        /// Heartbeat table for monitoring
        #[arg(long)]
        heartbeat_table: Option<String>,
    },

    /// Validate data consistency between source and target
    ///
    /// Performs data validation checks to ensure consistency
    /// between source and target databases.
    Check {
        /// Source database URL
        #[arg(short, long)]
        source: Option<String>,

        /// Target database URL
        #[arg(short, long)]
        target: Option<String>,

        /// Check mode (full, sample, checksum)
        #[arg(short, long)]
        mode: Option<String>,

        /// Output directory for check results
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Migrate database structure/schema
    ///
    /// Transfers database schemas, including tables, indexes,
    /// constraints, and other structural elements.
    Struct {
        /// Source database URL
        #[arg(short, long)]
        source: Option<String>,

        /// Target database URL
        #[arg(short, long)]
        target: Option<String>,

        /// Include data in structure migration
        #[arg(long)]
        include_data: bool,

        /// Drop target objects if they exist
        #[arg(long)]
        drop_if_exists: bool,
    },

    /// Run data quality and consistency reviews
    ///
    /// Performs comprehensive data quality checks and generates
    /// detailed reports on data consistency and integrity.
    Review {
        /// Source database URL
        #[arg(short, long)]
        source: Option<String>,

        /// Target database URL
        #[arg(short, long)]
        target: Option<String>,

        /// Review depth (basic, detailed, comprehensive)
        #[arg(short, long)]
        depth: Option<String>,

        /// Generate HTML report
        #[arg(long)]
        html_report: bool,
    },

    /// Revise and fix data inconsistencies
    ///
    /// Automatically corrects data inconsistencies found during
    /// validation checks.
    Revise {
        /// Source database URL
        #[arg(short, long)]
        source: Option<String>,

        /// Target database URL
        #[arg(short, long)]
        target: Option<String>,

        /// Revision strategy (conservative, aggressive, manual)
        #[arg(long)]
        strategy: Option<String>,

        /// Backup before making changes
        #[arg(long)]
        backup: bool,
    },

    /// Validate configuration file
    ///
    /// Validates the configuration file syntax and settings
    /// without executing any data transfer operations.
    Validate {
        /// Show detailed validation results
        #[arg(short, long)]
        detailed: bool,
    },

    /// Display configuration information
    ///
    /// Shows the current configuration settings and
    /// validates connectivity to configured databases.
    Info {
        /// Show connection status
        #[arg(short, long)]
        connections: bool,

        /// Show performance metrics
        #[arg(short, long)]
        metrics: bool,
    },
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            config: DEFAULT_CONFIG_FILE.to_string(),
            log_level: None,
            dry_run: false,
            verbose: false,
            skip_validation: false,
            max_parallel: None,
            batch_size: None,
            command: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parsing() {
        // Test default CLI parsing
        let cli = Cli::try_parse_from(&["ape-dts"]).unwrap();
        assert_eq!(cli.config, DEFAULT_CONFIG_FILE);
        assert_eq!(cli.log_level, None);
        assert!(!cli.dry_run);
        assert!(!cli.verbose);
        assert!(!cli.skip_validation);
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_cli_with_global_options() {
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "--config", "custom.ini",
            "--log-level", "debug",
            "--verbose",
            "--dry-run",
            "--max-parallel", "8",
            "--batch-size", "1000"
        ]).unwrap();

        assert_eq!(cli.config, "custom.ini");
        assert_eq!(cli.log_level, Some("debug".to_string()));
        assert!(cli.verbose);
        assert!(cli.dry_run);
        assert_eq!(cli.max_parallel, Some(8));
        assert_eq!(cli.batch_size, Some(1000));
    }

    #[test]
    fn test_snapshot_command() {
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "snapshot",
            "--source", "mysql://src",
            "--target", "postgres://dst",
            "--batch-size", "500",
            "--parallel", "4"
        ]).unwrap();

        match cli.command {
            Some(Commands::Snapshot { source, target, batch_size, parallel }) => {
                assert_eq!(source, Some("mysql://src".to_string()));
                assert_eq!(target, Some("postgres://dst".to_string()));
                assert_eq!(batch_size, Some(500));
                assert_eq!(parallel, Some(4));
            }
            _ => panic!("Expected Snapshot command"),
        }
    }

    #[test]
    fn test_cdc_command() {
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "cdc",
            "--source", "mysql://src",
            "--target", "kafka://dst",
            "--position", "mysql-bin.000001:12345",
            "--heartbeat-table", "heartbeat"
        ]).unwrap();

        match cli.command {
            Some(Commands::Cdc { source, target, position, heartbeat_table }) => {
                assert_eq!(source, Some("mysql://src".to_string()));
                assert_eq!(target, Some("kafka://dst".to_string()));
                assert_eq!(position, Some("mysql-bin.000001:12345".to_string()));
                assert_eq!(heartbeat_table, Some("heartbeat".to_string()));
            }
            _ => panic!("Expected Cdc command"),
        }
    }

    #[test]
    fn test_check_command() {
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "check",
            "--source", "mysql://src",
            "--target", "postgres://dst",
            "--mode", "checksum",
            "--output", "/tmp/results"
        ]).unwrap();

        match cli.command {
            Some(Commands::Check { source, target, mode, output }) => {
                assert_eq!(source, Some("mysql://src".to_string()));
                assert_eq!(target, Some("postgres://dst".to_string()));
                assert_eq!(mode, Some("checksum".to_string()));
                assert_eq!(output, Some("/tmp/results".to_string()));
            }
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn test_struct_command() {
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "struct",
            "--source", "mysql://src",
            "--target", "postgres://dst",
            "--include-data",
            "--drop-if-exists"
        ]).unwrap();

        match cli.command {
            Some(Commands::Struct { source, target, include_data, drop_if_exists }) => {
                assert_eq!(source, Some("mysql://src".to_string()));
                assert_eq!(target, Some("postgres://dst".to_string()));
                assert!(include_data);
                assert!(drop_if_exists);
            }
            _ => panic!("Expected Struct command"),
        }
    }

    #[test]
    fn test_review_command() {
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "review",
            "--source", "mysql://src",
            "--target", "postgres://dst",
            "--depth", "comprehensive",
            "--html-report"
        ]).unwrap();

        match cli.command {
            Some(Commands::Review { source, target, depth, html_report }) => {
                assert_eq!(source, Some("mysql://src".to_string()));
                assert_eq!(target, Some("postgres://dst".to_string()));
                assert_eq!(depth, Some("comprehensive".to_string()));
                assert!(html_report);
            }
            _ => panic!("Expected Review command"),
        }
    }

    #[test]
    fn test_revise_command() {
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "revise",
            "--source", "mysql://src",
            "--target", "postgres://dst",
            "--strategy", "conservative",
            "--backup"
        ]).unwrap();

        match cli.command {
            Some(Commands::Revise { source, target, strategy, backup }) => {
                assert_eq!(source, Some("mysql://src".to_string()));
                assert_eq!(target, Some("postgres://dst".to_string()));
                assert_eq!(strategy, Some("conservative".to_string()));
                assert!(backup);
            }
            _ => panic!("Expected Revise command"),
        }
    }

    #[test]
    fn test_validate_command() {
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "validate",
            "--detailed"
        ]).unwrap();

        match cli.command {
            Some(Commands::Validate { detailed }) => {
                assert!(detailed);
            }
            _ => panic!("Expected Validate command"),
        }
    }

    #[test]
    fn test_info_command() {
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "info",
            "--connections",
            "--metrics"
        ]).unwrap();

        match cli.command {
            Some(Commands::Info { connections, metrics }) => {
                assert!(connections);
                assert!(metrics);
            }
            _ => panic!("Expected Info command"),
        }
    }

    #[test]
    fn test_help_text_generation() {
        let mut cmd = Cli::command();
        let help = cmd.render_help();
        let help_str = help.to_string();

        // Test that help contains expected content
        assert!(help_str.contains("ApeCloud Data Transfer Service"));
        assert!(help_str.contains("snapshot"));
        assert!(help_str.contains("cdc"));
        assert!(help_str.contains("check"));
        assert!(help_str.contains("struct"));
        assert!(help_str.contains("review"));
        assert!(help_str.contains("revise"));
        assert!(help_str.contains("validate"));
        assert!(help_str.contains("info"));
    }

    #[test]
    fn test_version_command() {
        let result = Cli::try_parse_from(&["ape-dts", "--version"]);
        // This will fail because --version causes the program to exit
        // but we can test that the command accepts it
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_arguments() {
        // Test invalid log level
        let result = Cli::try_parse_from(&[
            "ape-dts",
            "--log-level", "invalid_level"
        ]);
        assert!(result.is_ok()); // clap accepts any string for log level

        // Test invalid numeric values
        let result = Cli::try_parse_from(&[
            "ape-dts",
            "--max-parallel", "not_a_number"
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn test_command_combinations() {
        // Test global options with subcommands
        let cli = Cli::try_parse_from(&[
            "ape-dts",
            "--config", "test.ini",
            "--verbose",
            "--dry-run",
            "snapshot",
            "--source", "mysql://test"
        ]).unwrap();

        assert_eq!(cli.config, "test.ini");
        assert!(cli.verbose);
        assert!(cli.dry_run);

        match cli.command {
            Some(Commands::Snapshot { source, .. }) => {
                assert_eq!(source, Some("mysql://test".to_string()));
            }
            _ => panic!("Expected Snapshot command"),
        }
    }
}
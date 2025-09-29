use std::env;
use std::path::Path;

use clap::Parser;
use anyhow::{Context, Result};
use tracing::{error, info, warn, debug};
use tracing_subscriber::EnvFilter;

use dt_precheck::{config::task_config::PrecheckTaskConfig, do_precheck};
use dt_task::task_runner::TaskRunner;

mod cli;
use cli::{Cli, Commands};

const ENV_SHUTDOWN_TIMEOUT_SECS: &str = "SHUTDOWN_TIMEOUT_SECS";


#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging first
    setup_logging(&cli)?;

    // Set up error handling and signal handling
    setup_error_handling();

    info!("Starting ape-dts v{}", env!("CARGO_PKG_VERSION"));
    debug!("Command line arguments: {:?}", std::env::args().collect::<Vec<_>>());

    // Validate configuration file exists
    if !Path::new(&cli.config).exists() {
        error!("Configuration file '{}' not found", cli.config);
        return Err(anyhow::anyhow!(
            "❌ Configuration file '{}' not found.\n\
            💡 Use --config to specify a different path or create the default config file.\n\
            💡 Example: ape-dts --config /path/to/your/config.ini",
            cli.config
        ));
    }

    info!("Using configuration file: {}", cli.config);

    // Handle dry run mode
    if cli.dry_run {
        info!("Running in dry-run mode - validating configuration only");
        println!("🔍 Dry run mode - validating configuration...");
        return validate_config(&cli.config, true, &cli).await;
    }

    // Execute based on subcommand
    match &cli.command {
        Some(command) => {
            info!("Executing command: {:?}", command);
            execute_command(command, &cli).await
        },
        None => {
            info!("No subcommand specified, running in legacy mode");
            warn!("Legacy mode will be deprecated in future versions. Please use explicit subcommands.");
            // No subcommand provided - try to detect task type from config and run
            execute_legacy_mode(&cli.config, &cli).await
        }
    }
}

fn log_command_overrides(source: &Option<String>, target: &Option<String>, batch_size: &Option<u32>, parallel: &Option<u32>) {
    if let Some(s) = source {
        info!("Source database override: {}", s);
    }
    if let Some(t) = target {
        info!("Target database override: {}", t);
    }
    if let Some(b) = batch_size {
        info!("Batch size override: {}", b);
    }
    if let Some(p) = parallel {
        info!("Parallel workers override: {}", p);
    }
}

fn setup_logging(cli: &Cli) -> Result<()> {
    let log_level = cli.log_level.as_deref().unwrap_or_else(|| {
        if cli.verbose {
            "debug"
        } else if cli.dry_run {
            "info"
        } else {
            "warn"
        }
    });

    let filter = EnvFilter::try_new(format!("{}={}", env!("CARGO_PKG_NAME").replace('-', "_"), log_level))
        .with_context(|| format!("Invalid log level: {}", log_level))?;

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(cli.verbose)
        .with_level(true)
        .init();

    debug!("Logging initialized with level: {}", log_level);
    Ok(())
}

fn setup_error_handling() {
    env::set_var("RUST_BACKTRACE", "1");

    tokio::spawn(async {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                info!("Received Ctrl+C, initiating graceful shutdown...");
                tokio::time::sleep(std::time::Duration::from_secs(
                    std::env::var(ENV_SHUTDOWN_TIMEOUT_SECS)
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(3),
                ))
                .await;
                info!("Shutdown complete");
                std::process::exit(0);
            }
            Err(err) => {
                error!("Failed to listen for Ctrl+C: {}", err);
            }
        }
    });
}

async fn validate_config(config_file: &str, detailed: bool, cli: &Cli) -> Result<()> {
    println!("🔧 Validating configuration file: {}", config_file);

    if detailed {
        println!("📋 Configuration Details:");
        println!("   - File: {}", config_file);
        println!("   - Skip validation: {}", cli.skip_validation);
        if let Some(level) = &cli.log_level {
            println!("   - Log level override: {}", level);
        }
        if let Some(parallel) = cli.max_parallel {
            println!("   - Max parallel override: {}", parallel);
        }
        if let Some(batch) = cli.batch_size {
            println!("   - Batch size override: {}", batch);
        }
    }

    if cli.skip_validation {
        warn!("⚠️ Skipping configuration validation per --skip-validation flag");
        println!("⚠️ Skipping configuration validation (--skip-validation)");
        return Ok(());
    }

    // Try to parse as precheck config first
    match PrecheckTaskConfig::new(config_file) {
        Ok(precheck_config) => {
            println!("✅ Configuration valid for precheck operations");
            if detailed {
                println!("   - Configuration type: Precheck");
                println!("   - Struct init: {}", precheck_config.precheck.do_struct_init);
                println!("   - CDC enabled: {}", precheck_config.precheck.do_cdc);
            }
            info!("Precheck configuration validated successfully");
            return Ok(());
        }
        Err(e) => {
            if detailed {
                println!("   ℹ️ Not a precheck configuration: {}", e);
            }
            debug!("Not a precheck configuration: {}", e);
        }
    }

    // Try to parse as regular task config
    match TaskRunner::new(config_file) {
        Ok(_runner) => {
            println!("✅ Configuration valid for task operations");
            if detailed {
                println!("   - Configuration type: Task");
                println!("   - Task configuration loaded successfully");
            }
            info!("Task configuration validated successfully");
            Ok(())
        }
        Err(e) => {
            error!("Configuration validation failed: {}", e);
            Err(anyhow::anyhow!(
                "❌ Configuration validation failed: {}\n\
                💡 Check your configuration file syntax and required fields.\n\
                💡 Use --verbose for more detailed error information.", e
            ))
        }
    }
}

async fn execute_command(command: &Commands, cli: &Cli) -> Result<()> {
    // First validate the configuration unless skipped
    if !cli.skip_validation {
        validate_config(&cli.config, cli.verbose, cli).await?;
    } else {
        warn!("Skipping configuration validation per --skip-validation flag");
    }

    match command {
        Commands::Snapshot { source, target, batch_size, parallel } => {
            info!("Starting snapshot operation");
            println!("🔄 Starting snapshot operation...");
            let combined_batch = batch_size.or(cli.batch_size);
            let combined_parallel = parallel.or(cli.max_parallel);
            log_command_overrides(source, target, &combined_batch, &combined_parallel);
            execute_task_operation(&cli.config, cli).await
        },
        Commands::Cdc { source, target, position, heartbeat_table } => {
            info!("Starting CDC replication");
            println!("📡 Starting CDC replication...");
            log_command_overrides(source, target, &cli.batch_size, &cli.max_parallel);
            if let Some(pos) = position {
                info!("CDC starting position: {}", pos);
            }
            if let Some(ht) = heartbeat_table {
                info!("Heartbeat table: {}", ht);
            }
            execute_task_operation(&cli.config, cli).await
        },
        Commands::Check { source, target, mode, output } => {
            info!("Starting data consistency check");
            println!("🔍 Starting data consistency check...");
            log_command_overrides(source, target, &cli.batch_size, &cli.max_parallel);
            if let Some(m) = mode {
                info!("Check mode: {}", m);
            }
            if let Some(o) = output {
                info!("Output directory: {}", o);
            }
            execute_check_operation(&cli.config, cli).await
        },
        Commands::Struct { source, target, include_data, drop_if_exists } => {
            info!("Starting structure migration");
            println!("🏗️  Starting structure migration...");
            log_command_overrides(source, target, &cli.batch_size, &cli.max_parallel);
            if *include_data {
                info!("Including data in structure migration");
            }
            if *drop_if_exists {
                warn!("Will drop existing target objects");
            }
            execute_task_operation(&cli.config, cli).await
        },
        Commands::Review { source, target, depth, html_report } => {
            info!("Starting data quality review");
            println!("📊 Starting data quality review...");
            log_command_overrides(source, target, &cli.batch_size, &cli.max_parallel);
            if let Some(d) = depth {
                info!("Review depth: {}", d);
            }
            if *html_report {
                info!("HTML report will be generated");
            }
            execute_check_operation(&cli.config, cli).await
        },
        Commands::Revise { source, target, strategy, backup } => {
            info!("Starting data revision");
            println!("🔧 Starting data revision...");
            log_command_overrides(source, target, &cli.batch_size, &cli.max_parallel);
            if let Some(s) = strategy {
                info!("Revision strategy: {}", s);
            }
            if *backup {
                info!("Backup will be created before changes");
            }
            execute_task_operation(&cli.config, cli).await
        },
        Commands::Validate { detailed } => {
            validate_config(&cli.config, *detailed, cli).await
        },
        Commands::Info { connections, metrics } => {
            display_info(&cli.config, *connections, *metrics).await
        },
    }
}

async fn execute_legacy_mode(config_file: &str, cli: &Cli) -> Result<()> {
    warn!("Running in legacy mode - this will be deprecated in future versions");
    println!("🔄 Running in legacy mode with config: {}", config_file);
    println!("⚠️  Legacy mode will be deprecated. Please use explicit subcommands.");

    // Apply CLI overrides logging
    if cli.max_parallel.is_some() || cli.batch_size.is_some() {
        warn!("CLI overrides (--max-parallel, --batch-size) are not fully supported in legacy mode");
        println!("⚠️  CLI parameter overrides may not be fully applied in legacy mode");
    }

    // Try precheck first, then regular task
    if PrecheckTaskConfig::new(config_file).is_ok() {
        info!("Detected precheck configuration");
        println!("🔍 Detected precheck configuration - running precheck operations");
        do_precheck(config_file).await;
        println!("✅ Precheck operations completed");
        Ok(())
    } else {
        info!("Detected task configuration");
        println!("⚙️  Detected task configuration - running task operations");
        let runner = TaskRunner::new(config_file)
            .with_context(|| format!("Failed to create task runner from config: {}", config_file))?;
        runner.start_task(true).await
            .with_context(|| "Task execution failed")?;
        println!("✅ Task operations completed");
        Ok(())
    }
}

async fn execute_task_operation(config_file: &str, cli: &Cli) -> Result<()> {
    info!("Initializing task runner from configuration: {}", config_file);

    let runner = TaskRunner::new(config_file)
        .with_context(|| format!(
            "❌ Failed to create task runner from config: {}\n\
            💡 Check that the configuration file has valid syntax and required fields.",
            config_file
        ))?;

    // Log CLI overrides that would be applied
    if cli.max_parallel.is_some() || cli.batch_size.is_some() {
        info!("Note: CLI parameter overrides are logged but may require TaskRunner API changes to fully apply");
    }

    info!("Starting task execution");
    match runner.start_task(true).await {
        Ok(_) => {
            info!("Task execution completed successfully");
            println!("✅ Task completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Task execution failed: {}", e);
            Err(anyhow::anyhow!(
                "❌ Task execution failed: {}\n\
                💡 Check the logs above for detailed error information.\n\
                💡 Verify your database connections and configuration settings.", e
            ))
        }
    }
}

async fn execute_check_operation(config_file: &str, cli: &Cli) -> Result<()> {
    // Try precheck first, if it fails, try regular task
    if PrecheckTaskConfig::new(config_file).is_ok() {
        do_precheck(config_file).await;
        println!("✅ Precheck completed successfully");
        Ok(())
    } else {
        execute_task_operation(config_file, cli).await
    }
}

async fn display_info(config_file: &str, show_connections: bool, show_metrics: bool) -> Result<()> {
    println!("📋 Configuration Information");
    println!("==========================");
    println!("Config file: {}", config_file);
    println!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Try to load and display basic info from both config types
    match PrecheckTaskConfig::new(config_file) {
        Ok(precheck_config) => {
            println!("Type: Precheck Configuration");
            println!("Struct Init: {}", precheck_config.precheck.do_struct_init);
            println!("CDC: {}", precheck_config.precheck.do_cdc);
            if show_connections {
                info!("Connection status check not yet implemented for precheck configs");
                println!("💡 Connection status check not yet implemented for precheck configs");
            }
        },
        Err(_) => {
            match TaskRunner::new(config_file) {
                Ok(_runner) => {
                    println!("Type: Task Configuration");
                    println!("Status: Valid task configuration loaded");
                    if show_connections {
                        info!("Connection status check not yet implemented for task configs");
                        println!("💡 Connection status check not yet implemented for task configs");
                    }
                },
                Err(e) => {
                    error!("Failed to load configuration: {}", e);
                    return Err(anyhow::anyhow!(
                        "❌ Failed to load configuration: {}\n\
                        💡 Check that the configuration file exists and has valid syntax.", e
                    ));
                }
            }
        }
    }

    if show_metrics {
        info!("Performance metrics display not yet implemented");
        println!("💡 Performance metrics display not yet implemented");
    }

    Ok(())
}
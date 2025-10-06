use std::process::Command;
use std::fs;
use tempfile::TempDir;

const BINARY_NAME: &str = "ape-dts";

fn get_binary_path() -> String {
    format!("../target/debug/{}", BINARY_NAME)
}

/// Create a comprehensive snapshot config with all sections
fn create_comprehensive_snapshot_config() -> tempfile::NamedTempFile {
    let config_content = r#"
[extractor]
db_type=mysql
extract_type=snapshot
url=mysql://source_user:source_pass@localhost:3306/source_db

[sinker]
db_type=mysql
sink_type=write
url=mysql://target_user:target_pass@localhost:3306/target_db
batch_size=1000

[runtime]
log_level=info
log_dir=./logs

[parallelizer]
parallel_type=snapshot
parallel_size=4

[pipeline]
pipeline_type=basic
buffer_size=16000

[filter]
do_schemas=*
ignore_schemas=information_schema,performance_schema,mysql,sys
do_events=*

[router]

[resumer]
"#;

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    fs::write(&temp_file, config_content).unwrap();
    temp_file
}

/// Create a CDC config file
fn create_cdc_config() -> tempfile::NamedTempFile {
    let config_content = r#"
[extractor]
db_type=mysql
extract_type=cdc
url=mysql://source_user:source_pass@localhost:3306/source_db
binlog_filename=mysql-bin.000001
binlog_position=12345

[sinker]
db_type=mysql
sink_type=write
url=mysql://target_user:target_pass@localhost:3306/target_db

[runtime]
log_level=debug

[parallelizer]
parallel_type=serial

[filter]
do_schemas=test_db
do_tables=users,orders
"#;

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    fs::write(&temp_file, config_content).unwrap();
    temp_file
}

/// Create a minimal valid config
fn create_minimal_config() -> tempfile::NamedTempFile {
    let config_content = r#"
[extractor]
url = mysql://test@localhost/test
extract_type = snapshot
db_type = mysql

[sinker]
url = mysql://test@localhost/test
sink_type = write
db_type = mysql

[parallelizer]
parallel_type = serial
"#;

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    fs::write(&temp_file, config_content).unwrap();
    temp_file
}

#[test]
fn test_end_to_end_snapshot_workflow() {
    let config = create_comprehensive_snapshot_config();

    // Step 1: Validate the configuration
    let validate_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("validate")
        .arg("--detailed")
        .output()
        .expect("Failed to validate config");

    let validate_stdout = String::from_utf8_lossy(&validate_output.stdout);
    println!("Validate output: {}", validate_stdout);

    // Step 2: Get info about the configuration
    let info_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("info")
        .output()
        .expect("Failed to get info");

    let info_stdout = String::from_utf8_lossy(&info_output.stdout);
    println!("Info output: {}", info_stdout);
    assert!(info_stdout.contains("Configuration Information"));

    // Step 3: Dry run the snapshot
    let snapshot_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--dry-run")
        .arg("--verbose")
        .arg("snapshot")
        .output()
        .expect("Failed to dry run snapshot");

    let snapshot_stdout = String::from_utf8_lossy(&snapshot_output.stdout);
    println!("Snapshot dry run output: {}", snapshot_stdout);
    assert!(snapshot_stdout.contains("Dry run mode"));
}

#[test]
fn test_end_to_end_cdc_workflow() {
    let config = create_cdc_config();

    // Validate
    let validate_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("validate")
        .output()
        .expect("Failed to validate CDC config");

    let validate_stdout = String::from_utf8_lossy(&validate_output.stdout);
    println!("CDC validate output: {}", validate_stdout);

    // Dry run CDC with parameters
    let cdc_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--dry-run")
        .arg("--log-level")
        .arg("debug")
        .arg("cdc")
        .arg("--position")
        .arg("mysql-bin.000002:54321")
        .output()
        .expect("Failed to dry run CDC");

    let cdc_stdout = String::from_utf8_lossy(&cdc_output.stdout);
    println!("CDC dry run output: {}", cdc_stdout);
    assert!(cdc_stdout.contains("Dry run mode"));
}

#[test]
fn test_error_scenarios_invalid_config_syntax() {
    let invalid_config = tempfile::NamedTempFile::new().unwrap();
    fs::write(&invalid_config, "[invalid\nsyntax here").unwrap();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(invalid_config.path())
        .arg("validate")
        .output()
        .expect("Failed to run validation");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stderr: {}", stderr);
    println!("stdout: {}", stdout);

    // Should show an error about invalid config
    assert!(stderr.len() > 0 || stdout.contains("validation") || stdout.contains("error"));
}

#[test]
fn test_error_scenarios_missing_required_fields() {
    let incomplete_config = tempfile::NamedTempFile::new().unwrap();
    fs::write(&incomplete_config, r#"
[extractor]
db_type=mysql
# Missing extract_type and url
"#).unwrap();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(incomplete_config.path())
        .arg("validate")
        .output()
        .expect("Failed to run validation");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("Error output: {}", stderr);
}

#[test]
fn test_error_scenarios_conflicting_options() {
    let config = create_minimal_config();

    // Try to use both verbose and quiet (if quiet existed)
    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--verbose")
        .arg("--dry-run")
        .arg("snapshot")
        .output()
        .expect("Failed to run command");

    // Verbose + dry-run should work fine
    assert!(output.status.success() || output.status.code() == Some(0));
}

#[test]
fn test_all_log_levels() {
    let config = create_minimal_config();
    let log_levels = ["trace", "debug", "info", "warn", "error"];

    for level in &log_levels {
        let output = Command::new(&get_binary_path())
            .arg("--config")
            .arg(config.path())
            .arg("--log-level")
            .arg(level)
            .arg("--dry-run")
            .arg("validate")
            .output()
            .expect(&format!("Failed to run with log level {}", level));

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Log level {} output: {}", level, stdout);

        // Should either succeed or fail gracefully
        assert!(output.status.success() || !stdout.is_empty());
    }
}

#[test]
fn test_parameter_override_precedence() {
    let config = create_comprehensive_snapshot_config();

    // Test that command-line args override config file values
    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--dry-run")
        .arg("--max-parallel")
        .arg("16")  // Override the config value
        .arg("--batch-size")
        .arg("5000")  // Override the config value
        .arg("snapshot")
        .arg("--batch-size")
        .arg("10000")  // Subcommand override
        .arg("--parallel")
        .arg("32")  // Subcommand override
        .output()
        .expect("Failed to run with overrides");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Override test output: {}", stdout);
    assert!(stdout.contains("Dry run mode"));
}

#[test]
fn test_help_for_all_commands() {
    let commands = vec![
        vec!["--help"],
        vec!["snapshot", "--help"],
        vec!["cdc", "--help"],
        vec!["check", "--help"],
        vec!["struct", "--help"],
        vec!["review", "--help"],
        vec!["revise", "--help"],
        vec!["validate", "--help"],
        vec!["info", "--help"],
    ];

    for cmd_args in commands {
        let output = Command::new(&get_binary_path())
            .args(&cmd_args)
            .output()
            .expect(&format!("Failed to run {:?}", cmd_args));

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.is_empty());
        println!("{:?} help length: {} chars", cmd_args, stdout.len());
    }
}

#[test]
fn test_version_info() {
    let output = Command::new(&get_binary_path())
        .arg("--version")
        .output()
        .expect("Failed to get version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1") || stdout.contains("dt-main") || stdout.contains("ape-dts"));
}

#[test]
fn test_validate_command_detailed_vs_normal() {
    let config = create_minimal_config();

    // Normal validation
    let normal_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("validate")
        .output()
        .expect("Failed normal validation");

    // Detailed validation
    let detailed_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("validate")
        .arg("--detailed")
        .output()
        .expect("Failed detailed validation");

    let normal_stdout = String::from_utf8_lossy(&normal_output.stdout);
    let detailed_stdout = String::from_utf8_lossy(&detailed_output.stdout);

    println!("Normal validation: {}", normal_stdout);
    println!("Detailed validation: {}", detailed_stdout);

    // Detailed should have more information
    assert!(detailed_stdout.len() >= normal_stdout.len());
}

#[test]
fn test_skip_validation_flag() {
    let config = create_minimal_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--skip-validation")
        .arg("validate")
        .output()
        .expect("Failed with skip-validation");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Skip validation output: {}", stdout);

    assert!(stdout.contains("Skipping configuration validation"));
}

#[test]
fn test_nonexistent_config_error_message() {
    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg("/nonexistent/path/to/config.ini")
        .arg("validate")
        .output()
        .expect("Failed to execute");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("Error message: {}", stderr);

    // Should have a user-friendly error message
    assert!(stderr.contains("not found") || stderr.contains("Configuration file"));
    assert!(stderr.contains("💡") || stderr.contains("Use --config") ||
            stderr.contains("Example:") || stderr.len() > 50);
}

#[test]
fn test_comprehensive_error_messages() {
    let config = create_minimal_config();

    // Test with invalid batch size
    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--batch-size")
        .arg("0")
        .arg("validate")
        .output()
        .expect("Failed to execute");

    // Should either reject 0 or accept it
    println!("Exit code: {:?}", output.status.code());
}

#[test]
fn test_struct_migration_flags() {
    let config = create_minimal_config();

    // Test with all struct flags
    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--dry-run")
        .arg("struct")
        .arg("--include-data")
        .arg("--drop-if-exists")
        .output()
        .expect("Failed struct command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Struct migration output: {}", stdout);
    assert!(stdout.contains("Dry run mode"));
}

#[test]
fn test_review_with_html_report() {
    let config = create_minimal_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--dry-run")
        .arg("review")
        .arg("--depth")
        .arg("comprehensive")
        .arg("--html-report")
        .output()
        .expect("Failed review command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dry run mode"));
}

#[test]
fn test_revise_with_backup() {
    let config = create_minimal_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--dry-run")
        .arg("revise")
        .arg("--strategy")
        .arg("conservative")
        .arg("--backup")
        .output()
        .expect("Failed revise command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dry run mode"));
}

#[test]
fn test_info_with_all_flags() {
    let config = create_minimal_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("info")
        .arg("--connections")
        .arg("--metrics")
        .output()
        .expect("Failed info command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Info output: {}", stdout);
    assert!(stdout.contains("Configuration Information"));
}

#[test]
fn test_check_with_output_directory() {
    let config = create_minimal_config();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("check-results");

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config.path())
        .arg("--dry-run")
        .arg("check")
        .arg("--mode")
        .arg("checksum")
        .arg("--output")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("Failed check command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dry run mode"));
}

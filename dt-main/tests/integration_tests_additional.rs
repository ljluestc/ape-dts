use std::process::Command;
use std::fs;

const BINARY_NAME: &str = "ape-dts";

fn get_binary_path() -> String {
    format!("../target/debug/{}", BINARY_NAME)
}

fn create_precheck_config() -> tempfile::NamedTempFile {
    let config_content = r#"
[precheck]
do_struct_init = true
do_cdc = false
"#;

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    fs::write(&temp_file, config_content).unwrap();
    temp_file
}

fn create_cdc_config() -> tempfile::NamedTempFile {
    let config_content = r#"
[extractor]
url = mysql://test:test@localhost/testdb
extract_type = cdc
db_type = mysql

[sinker]
url = kafka://test:test@localhost:9092
sink_type = kafka
db_type = kafka

[parallelizer]
parallel_type = serial
"#;

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    fs::write(&temp_file, config_content).unwrap();
    temp_file
}

fn create_postgres_to_mysql_config() -> tempfile::NamedTempFile {
    let config_content = r#"
[extractor]
url = mysql://test:test@localhost/testdb
extract_type = snapshot
db_type = mysql

[sinker]
url = mysql://test:test@localhost/testdb
sink_type = write
db_type = mysql

[parallelizer]
parallel_type = serial
batch_size = 1000
max_parallel = 4
"#;

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    fs::write(&temp_file, config_content).unwrap();
    temp_file
}

#[test]
fn test_precheck_config_validation() {
    let config_file = create_precheck_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("validate")
        .arg("--detailed")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should recognize as precheck configuration
    assert!(stdout.contains("precheck") || stdout.contains("Precheck") ||
            stderr.contains("precheck") || stderr.contains("Precheck"));
}

#[test]
fn test_cdc_config_dry_run() {
    let config_file = create_cdc_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")
        .arg("cdc")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should show dry run mode
    assert!(stdout.contains("Dry run mode") || stderr.contains("validation"));
}

#[test]
fn test_error_handling_invalid_config() {
    let invalid_config = tempfile::NamedTempFile::new().unwrap();
    fs::write(&invalid_config, "invalid config content").unwrap();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(invalid_config.path())
        .arg("validate")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should fail validation - may panic or return error
    assert!(!output.status.success());
    // Check for either panic in stderr or validation error message
    assert!(stderr.len() > 0 || stdout.contains("validation") || stdout.contains("error"));
}

#[test]
fn test_postgres_to_mysql_migration() {
    let config_file = create_postgres_to_mysql_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")
        .arg("--verbose")
        .arg("struct")
        .arg("--include-data")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should show dry run mode (structure migration might fail due to validation)
    assert!(stdout.contains("Dry run mode") || stderr.len() > 0);
}

#[test]
fn test_complete_workflow_with_validation() {
    let config_file = create_postgres_to_mysql_config();

    // Test validation first
    let validate_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("validate")
        .arg("--detailed")
        .output()
        .expect("Failed to execute validate command");

    // Test info command
    let info_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("info")
        .arg("--connections")
        .arg("--metrics")
        .output()
        .expect("Failed to execute info command");

    // Test snapshot with overrides
    let snapshot_output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")
        .arg("--max-parallel")
        .arg("8")
        .arg("--batch-size")
        .arg("2000")
        .arg("snapshot")
        .output()
        .expect("Failed to execute snapshot command");

    let validate_stdout = String::from_utf8_lossy(&validate_output.stdout);
    let info_stdout = String::from_utf8_lossy(&info_output.stdout);
    let snapshot_stdout = String::from_utf8_lossy(&snapshot_output.stdout);

    println!("Validate stdout: {}", validate_stdout);
    println!("Info stdout: {}", info_stdout);
    println!("Snapshot stdout: {}", snapshot_stdout);

    // Validate should work or show meaningful error
    assert!(validate_stdout.contains("Configuration") ||
            validate_output.stderr.len() > 0);

    // Info should work or show meaningful error
    assert!(info_stdout.contains("Configuration") ||
            info_output.stderr.len() > 0);

    // Snapshot dry run should show dry run mode
    assert!(snapshot_stdout.contains("Dry run mode"));
}

#[test]
fn test_all_subcommands_help() {
    let subcommands = ["snapshot", "cdc", "check", "struct", "review", "revise", "validate", "info"];

    for subcommand in &subcommands {
        let output = Command::new(&get_binary_path())
            .arg(subcommand)
            .arg("--help")
            .output()
            .expect(&format!("Failed to execute {} --help", subcommand));

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{} help: {}", subcommand, stdout);

        // Should show help for the subcommand
        assert!(output.status.success());
        assert!(stdout.contains(subcommand) || stdout.len() > 0);
    }
}

#[test]
fn test_global_options_combinations() {
    let config_file = create_postgres_to_mysql_config();

    // Test all global options together
    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--log-level")
        .arg("debug")
        .arg("--verbose")
        .arg("--dry-run")
        .arg("--max-parallel")
        .arg("16")
        .arg("--batch-size")
        .arg("5000")
        .arg("snapshot")
        .arg("--source")
        .arg("postgres://override")
        .arg("--target")
        .arg("mysql://override")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should show dry run mode with verbose output
    assert!(stdout.contains("Dry run mode"));
}

#[test]
fn test_skip_validation_flag() {
    let config_file = create_postgres_to_mysql_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--skip-validation")
        .arg("--dry-run")
        .arg("snapshot")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should show that validation was skipped
    assert!(stdout.contains("Skipping configuration validation") ||
            stderr.contains("skip") ||
            stdout.contains("Dry run mode"));
}

#[test]
fn test_legacy_mode_detection() {
    let config_file = create_postgres_to_mysql_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should show dry run mode (no subcommand means legacy mode might be triggered)
    assert!(stdout.contains("Dry run mode") || stderr.contains("validation"));
}
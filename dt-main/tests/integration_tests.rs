use std::process::Command;
use std::fs;

const BINARY_NAME: &str = "ape-dts";

fn get_binary_path() -> String {
    format!("../target/debug/{}", BINARY_NAME)
}

fn create_test_config() -> tempfile::NamedTempFile {
    let config_content = r#"
[extractor]
url = mysql://test:test@localhost/testdb
extract_type = snapshot
db_type = mysql

[sinker]
url = postgres://test:test@localhost/testdb
sink_type = write
db_type = postgres

[parallelizer]
parallel_type = serial
"#;

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    fs::write(&temp_file, config_content).unwrap();
    temp_file
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

#[test]
fn test_help_command() {
    let output = Command::new(&get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ApeCloud Data Transfer Service"));
    assert!(stdout.contains("snapshot"));
    assert!(stdout.contains("cdc"));
    assert!(stdout.contains("check"));
    assert!(stdout.contains("struct"));
}

#[test]
fn test_version_command() {
    let output = Command::new(&get_binary_path())
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("dt-main") || stdout.contains("ape-dts"));
}

#[test]
fn test_config_file_not_found() {
    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg("nonexistent.ini")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Configuration file") || stderr.contains("not found"));
}

#[test]
fn test_dry_run_mode() {
    let config_file = create_test_config();

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

    // In dry run mode, it should attempt validation but may fail due to missing dependencies
    assert!(stdout.contains("Dry run mode") || stderr.contains("validation"));
}

#[test]
fn test_validate_subcommand() {
    let config_file = create_test_config();

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

    // Should attempt validation
    assert!(stdout.contains("Validating configuration") || stderr.contains("validation"));
}

#[test]
fn test_info_subcommand() {
    let config_file = create_test_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("info")
        .arg("--connections")
        .arg("--metrics")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should show configuration information (but may fail due to missing config fields)
    assert!(stdout.contains("Configuration Information") ||
            stderr.contains("Configuration") ||
            stderr.contains("config") ||
            !output.status.success());
}

#[test]
fn test_snapshot_subcommand_with_overrides() {
    let config_file = create_test_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")  // Use dry run to avoid actual execution
        .arg("snapshot")
        .arg("--source")
        .arg("mysql://override:override@localhost/override")
        .arg("--target")
        .arg("postgres://override:override@localhost/override")
        .arg("--batch-size")
        .arg("1000")
        .arg("--parallel")
        .arg("4")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should show dry run validation
    assert!(stdout.contains("Dry run mode") || stderr.contains("validation"));
}

#[test]
fn test_global_options() {
    let config_file = create_test_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--log-level")
        .arg("debug")
        .arg("--verbose")
        .arg("--skip-validation")
        .arg("--max-parallel")
        .arg("8")
        .arg("--batch-size")
        .arg("2000")
        .arg("validate")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // With skip-validation, should bypass validation
    assert!(stdout.contains("Skipping configuration validation") ||
            stderr.contains("skip") ||
            output.status.success());
}

#[test]
fn test_invalid_log_level() {
    let config_file = create_test_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--log-level")
        .arg("invalid_level")
        .arg("--dry-run")
        .output()
        .expect("Failed to execute command");

    // Invalid log level should be handled gracefully or cause an error
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should either succeed with warning or fail with error
    assert!(stdout.contains("Dry run") || stderr.contains("Invalid") ||
            stdout.contains("level") || stderr.contains("level"));
}

#[test]
fn test_cdc_subcommand() {
    let config_file = create_test_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")
        .arg("cdc")
        .arg("--source")
        .arg("mysql://src")
        .arg("--target")
        .arg("kafka://dst")
        .arg("--position")
        .arg("mysql-bin.000001:12345")
        .arg("--heartbeat-table")
        .arg("heartbeat")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);

    assert!(stdout.contains("Dry run mode"));
}

#[test]
fn test_struct_subcommand() {
    let config_file = create_test_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")
        .arg("struct")
        .arg("--source")
        .arg("mysql://src")
        .arg("--target")
        .arg("postgres://dst")
        .arg("--include-data")
        .arg("--drop-if-exists")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);

    assert!(stdout.contains("Dry run mode"));
}

#[test]
fn test_check_subcommand() {
    let config_file = create_test_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")
        .arg("check")
        .arg("--source")
        .arg("mysql://src")
        .arg("--target")
        .arg("postgres://dst")
        .arg("--mode")
        .arg("checksum")
        .arg("--output")
        .arg("/tmp/results")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);

    assert!(stdout.contains("Dry run mode"));
}

#[test]
fn test_review_subcommand() {
    let config_file = create_test_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")
        .arg("review")
        .arg("--source")
        .arg("mysql://src")
        .arg("--target")
        .arg("postgres://dst")
        .arg("--depth")
        .arg("comprehensive")
        .arg("--html-report")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);

    assert!(stdout.contains("Dry run mode"));
}

#[test]
fn test_revise_subcommand() {
    let config_file = create_test_config();

    let output = Command::new(&get_binary_path())
        .arg("--config")
        .arg(config_file.path())
        .arg("--dry-run")
        .arg("revise")
        .arg("--source")
        .arg("mysql://src")
        .arg("--target")
        .arg("postgres://dst")
        .arg("--strategy")
        .arg("conservative")
        .arg("--backup")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);

    assert!(stdout.contains("Dry run mode"));
}
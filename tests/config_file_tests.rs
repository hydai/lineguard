use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_config_file_ignore_patterns() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("good.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("bad.txt"), "content  \n").unwrap();
    std::fs::write(temp_dir.path().join("ignore.txt"), "content  \n").unwrap();

    // Create config file
    let config_content = r#"
ignore_patterns = ["ignore.txt"]
"#;
    std::fs::write(temp_dir.path().join(".lineguardrc"), config_content).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path());
    cmd.arg("--recursive");
    cmd.arg("--config")
        .arg(temp_dir.path().join(".lineguardrc"));
    cmd.arg("--format").arg("json");

    cmd.assert()
        .failure()  // Should fail due to bad.txt
        .stdout(predicate::str::contains("\"files_checked\": 3"))  // good.txt, bad.txt, and .lineguardrc
        .stdout(predicate::str::contains("bad.txt"))
        .stdout(predicate::str::contains("ignore.txt").not());
}

#[test]
fn test_config_file_extensions() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("test.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("test.rs"), "content  \n").unwrap();
    std::fs::write(temp_dir.path().join("test.py"), "content\n").unwrap();

    // Create config file
    let config_content = r#"
file_extensions = ["txt", "py"]
"#;
    std::fs::write(temp_dir.path().join(".lineguardrc"), config_content).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path());
    cmd.arg("--recursive");
    cmd.arg("--config")
        .arg(temp_dir.path().join(".lineguardrc"));
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 3")); // .txt, .py files, and .lineguardrc
}

#[test]
fn test_config_file_check_options() {
    let temp_dir = TempDir::new().unwrap();

    // Create test file with both issues
    std::fs::write(temp_dir.path().join("test.txt"), "content  ").unwrap(); // Trailing space and no newline

    // Create config file disabling newline check
    let config_content = r#"
[checks]
newline_ending = false
trailing_spaces = true
"#;
    std::fs::write(temp_dir.path().join(".lineguardrc"), config_content).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path().join("test.txt"));
    cmd.arg("--config")
        .arg(temp_dir.path().join(".lineguardrc"));
    cmd.arg("--format").arg("json");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("trailing_space"))
        .stdout(predicate::str::contains("missing_newline").not());
}

#[test]
fn test_config_file_in_parent_directory() {
    let temp_dir = TempDir::new().unwrap();
    let sub_dir = temp_dir.path().join("subdir");
    std::fs::create_dir(&sub_dir).unwrap();

    // Create test file in subdirectory
    std::fs::write(sub_dir.join("test.txt"), "content  \n").unwrap();

    // Create config file in parent directory
    let config_content = r#"
file_extensions = ["md"]  # Only check .md files
"#;
    std::fs::write(temp_dir.path().join(".lineguardrc"), config_content).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&sub_dir); // Run from subdirectory
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("No files found to check")); // No .md files found
}

#[test]
fn test_config_file_cli_overrides() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("test.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("test.rs"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("ignore.txt"), "content  \n").unwrap();

    // Create config file
    let config_content = r#"
file_extensions = ["txt"]
ignore_patterns = []
"#;
    std::fs::write(temp_dir.path().join(".lineguardrc"), config_content).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path());
    cmd.arg("--recursive");
    cmd.arg("--config")
        .arg(temp_dir.path().join(".lineguardrc"));
    cmd.arg("--extensions").arg("txt,rs"); // Override config
    cmd.arg("--ignore").arg("ignore.txt"); // Override config
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 3")); // Both .txt and .rs, and .lineguardrc, but not ignore.txt
}

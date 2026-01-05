use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_human_reporter_shows_checking_message() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("file1.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("file2.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("file3.txt"), "content\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Checking 3 files..."));
}

#[test]
fn test_human_reporter_no_message_for_single_file() {
    let temp_dir = TempDir::new().unwrap();

    // Create single test file
    std::fs::write(temp_dir.path().join("file.txt"), "content\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("file.txt");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Checking").not());
}

#[test]
fn test_human_reporter_no_message_with_quiet_flag() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("file1.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("file2.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("file3.txt"), "content\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--quiet");

    cmd.assert().success().stdout(predicate::str::is_empty());
}

#[test]
fn test_human_reporter_no_message_with_json_format() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("file1.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("file2.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("file3.txt"), "content\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Checking").not())
        .stdout(predicate::str::contains("{"));
}

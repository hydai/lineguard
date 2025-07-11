use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_accepts_single_file_path() {
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().join("test.txt");
    std::fs::write(&temp_path, "test content\n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(&temp_path);
    cmd.arg("--format").arg("json");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("{"))
        .stdout(predicate::str::contains("\"files_checked\": 1"));
}

#[test]
fn test_cli_accepts_multiple_file_paths() {
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("file1.txt").arg("file2.txt").arg("file3.txt");
    cmd.assert().success();
}

#[test]
fn test_cli_shows_help() {
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("lineguard"));
}

#[test]
fn test_cli_shows_version() {
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("lineguard"));
}

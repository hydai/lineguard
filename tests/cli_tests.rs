use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_cli_accepts_single_file_path() {
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().join("test.txt");
    std::fs::write(&temp_path, "test content\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--format").arg("json");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("{"))
        .stdout(predicate::str::contains("\"files_checked\": 1"));
}

#[test]
fn test_cli_accepts_multiple_file_paths() {
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("file1.txt").arg("file2.txt").arg("file3.txt");
    cmd.assert().success();
}

#[test]
fn test_cli_shows_help() {
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("lineguard"));
}

#[test]
fn test_cli_shows_version() {
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("lineguard"));
}

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_ignore_pattern_simple() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("good.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("bad.txt"), "content  \n").unwrap();
    std::fs::write(temp_dir.path().join("ignore.txt"), "content  \n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--ignore").arg("ignore.txt");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .failure()  // Should fail due to bad.txt
        .stdout(predicate::str::contains("\"files_checked\": 2"))  // Only good.txt and bad.txt
        .stdout(predicate::str::contains("bad.txt"))
        .stdout(predicate::str::contains("ignore.txt").not());
}

#[test]
fn test_ignore_pattern_glob() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("test.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("test.log"), "content  \n").unwrap();
    std::fs::write(temp_dir.path().join("test.tmp"), "content  \n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--ignore").arg("*.log");
    cmd.arg("--ignore").arg("*.tmp");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 1")); // Only test.txt
}

#[test]
fn test_ignore_pattern_directory() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("good.txt"), "content\n").unwrap();
    std::fs::create_dir(temp_dir.path().join("node_modules")).unwrap();
    std::fs::write(temp_dir.path().join("node_modules/bad.txt"), "content  \n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--ignore").arg("**/node_modules/**");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 1")); // Only good.txt
}

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_extensions_parameter_single() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files with different extensions
    std::fs::write(temp_dir.path().join("test.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("test.rs"), "content  \n").unwrap();
    std::fs::write(temp_dir.path().join("test.py"), "content\n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path());
    cmd.arg("--recursive");
    cmd.arg("--extensions").arg("txt");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 1")); // Only test.txt
}

#[test]
fn test_extensions_parameter_multiple() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files with different extensions
    std::fs::write(temp_dir.path().join("test.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("test.rs"), "content  \n").unwrap();
    std::fs::write(temp_dir.path().join("test.py"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("test.md"), "content  \n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path());
    cmd.arg("--recursive");
    cmd.arg("--extensions").arg("txt,py");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 2")); // Only test.txt and test.py
}

#[test]
fn test_extensions_includes_hidden_files_by_default() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("test.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join(".hidden.txt"), "content  \n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path());
    cmd.arg("--recursive");
    cmd.arg("--extensions").arg("txt");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .failure()  // Should fail because .hidden.txt has issues
        .stdout(predicate::str::contains("\"files_checked\": 2"))  // Both files checked
        .stdout(predicate::str::contains(".hidden.txt")); // Hidden file is checked
}

#[test]
fn test_extensions_combines_with_ignore() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("test.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("ignore.txt"), "content  \n").unwrap();
    std::fs::write(temp_dir.path().join("test.rs"), "content\n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path());
    cmd.arg("--recursive");
    cmd.arg("--extensions").arg("txt");
    cmd.arg("--ignore").arg("ignore.txt");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 1"))  // Only test.txt
        .stdout(predicate::str::contains("ignore.txt").not());
}

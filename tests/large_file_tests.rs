use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_check_large_file_with_issues() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large.txt");

    // Create a large file (10MB) with issues
    let mut content = String::new();
    for i in 0..500_000 {
        if i % 1000 == 0 {
            content.push_str(&format!("Line {i} with trailing spaces  \n"));
        } else {
            content.push_str(&format!("Line {i}\n"));
        }
    }
    // Remove final newline to create an issue
    content.pop();
    std::fs::write(&file_path, content).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("large.txt");

    cmd.assert()
        .code(1) // Should exit with code 1 when issues are found
        .stdout(predicate::str::contains("Trailing spaces"))
        .stdout(predicate::str::contains("Missing newline"));
}

#[test]
fn test_fix_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large.txt");

    // Create a large file (5MB) with trailing spaces
    let mut content = String::new();
    for i in 0..250_000 {
        content.push_str(&format!("Line {i} with trailing spaces  \n"));
    }
    std::fs::write(&file_path, content).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("large.txt");
    cmd.arg("--fix");

    cmd.assert().success();

    // Verify file was fixed
    let fixed_content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!fixed_content.contains("  \n"));
    assert!(fixed_content.ends_with('\n'));
}

#[test]
fn test_memory_efficient_processing() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("huge.txt");

    // Create a very large file (50MB) - should still work without excessive memory
    let line = "This is a normal line without issues\n";
    let mut content = String::with_capacity(50 * 1024 * 1024);
    for _ in 0..(50 * 1024 * 1024 / line.len()) {
        content.push_str(line);
    }
    std::fs::write(&file_path, content).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("huge.txt");

    // Should complete successfully without running out of memory
    cmd.assert().success();
}

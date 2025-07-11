use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_fix_trailing_spaces() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with trailing spaces
    std::fs::write(&file_path, "line 1  \nline 2   \nline 3\n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");

    cmd.assert().success();

    // Check file was fixed
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line 1\nline 2\nline 3\n");
}

#[test]
fn test_fix_missing_newline() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file without newline at end
    std::fs::write(&file_path, "line 1\nline 2\nline 3").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");

    cmd.assert().success();

    // Check file was fixed
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line 1\nline 2\nline 3\n");
}

#[test]
fn test_fix_multiple_newlines() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with multiple newlines at end
    std::fs::write(&file_path, "line 1\nline 2\nline 3\n\n\n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");

    cmd.assert().success();

    // Check file was fixed
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line 1\nline 2\nline 3\n");
}

#[test]
fn test_fix_combined_issues() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with both trailing spaces and missing newline
    std::fs::write(&file_path, "line 1  \nline 2   \nline 3  ").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");

    cmd.assert().success();

    // Check file was fixed
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line 1\nline 2\nline 3\n");
}

#[test]
fn test_fix_with_quiet_flag() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    std::fs::write(&file_path, "line 1  \n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");
    cmd.arg("--quiet");

    cmd.assert().success().stdout(predicate::str::is_empty());
}

#[test]
fn test_fix_shows_fixed_files() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    std::fs::write(&file_path, "line 1  \n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fixed"))
        .stdout(predicate::str::contains("test.txt"));
}

#[test]
fn test_fix_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    let original_content = "line 1  \nline 2\n";
    std::fs::write(&file_path, original_content).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");
    cmd.arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Would fix"))
        .stdout(predicate::str::contains("test.txt"));

    // Check file was NOT modified
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, original_content);
}

#[test]
fn test_fix_respects_config() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with trailing spaces but no newline
    std::fs::write(&file_path, "line 1  ").unwrap();

    // Create config that disables newline check
    let config_content = r#"
[checks]
newline_ending = false
trailing_spaces = true
"#;
    std::fs::write(temp_dir.path().join(".lineguardrc"), config_content).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");

    cmd.assert().success();

    // Check only trailing spaces were fixed, not the missing newline
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line 1");
}

#[test]
fn test_fix_no_issues() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with no issues
    std::fs::write(&file_path, "line 1\nline 2\nline 3\n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fixed").not());
}

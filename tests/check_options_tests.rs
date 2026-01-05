use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_no_newline_check_flag() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file without newline at end
    std::fs::write(&file_path, "line 1\nline 2").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--no-newline-check");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Missing newline").not());
}

#[test]
fn test_no_trailing_space_flag() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with trailing spaces
    std::fs::write(&file_path, "line 1  \nline 2   \n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--no-trailing-space");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Trailing spaces").not());
}

#[test]
fn test_both_check_flags_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with both issues
    std::fs::write(&file_path, "line 1  \nline 2   ").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--no-newline-check");
    cmd.arg("--no-trailing-space");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("All files passed"));
}

#[test]
fn test_cli_flags_override_config_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with trailing spaces
    std::fs::write(&file_path, "line 1  \nline 2\n").unwrap();

    // Create config that enables trailing space check
    let config_content = r#"
[checks]
newline_ending = true
trailing_spaces = true
"#;
    std::fs::write(temp_dir.path().join(".lineguardrc"), config_content).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--no-trailing-space");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("All files passed"));
}

#[test]
fn test_check_flags_with_fix_mode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with both issues
    std::fs::write(&file_path, "line 1  \nline 2").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--fix");
    cmd.arg("--no-trailing-space");

    cmd.assert().success();

    // Check that only newline was fixed
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line 1  \nline 2\n"); // Trailing spaces remain
}

#[test]
fn test_check_flags_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create file with trailing spaces
    std::fs::write(&file_path, "line 1  \nline 2\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("test.txt");
    cmd.arg("--no-trailing-space");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"total_issues\": 0"));
}

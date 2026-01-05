use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::TempDir;

#[test]
fn test_main_with_no_files() {
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--quiet")
        .arg("nonexistent_pattern_*.txt")
        .assert()
        .success()
        .stdout("");
}

#[test]
fn test_main_with_invalid_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("invalid.toml");
    std::fs::write(&config_path, "invalid toml content [[[").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--config")
        .arg(config_path)
        .arg(".")
        .assert()
        .failure()
        .code(4);
}

#[test]
fn test_main_exit_codes() {
    let temp_dir = TempDir::new().unwrap();

    // Test exit code 0: no issues found
    let good_file = temp_dir.path().join("good.txt");
    std::fs::write(&good_file, "content\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(&good_file).assert().success().code(0);

    // Test exit code 1: issues found
    let bad_file = temp_dir.path().join("bad.txt");
    std::fs::write(&bad_file, "content").unwrap(); // Missing newline

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(&bad_file).assert().failure().code(1);
}

#[test]
fn test_main_with_fix_mode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("fixme.txt");
    std::fs::write(&file_path, "content").unwrap(); // Missing newline

    // Test dry run mode
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--fix")
        .arg("--dry-run")
        .arg(&file_path)
        .assert()
        .success();

    // Verify file wasn't changed
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "content");

    // Test actual fix
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--fix").arg(&file_path).assert().success();

    // Verify file was fixed
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "content\n");
}

#[test]
fn test_main_with_multiple_output_formats() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "content").unwrap(); // Missing newline

    // Test JSON format
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--format")
        .arg("json")
        .arg(&file_path)
        .assert()
        .failure()
        .stdout(predicates::str::contains("\"issues\""));

    // Test GitHub format
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--format")
        .arg("github")
        .arg(&file_path)
        .assert()
        .failure()
        .stdout(predicates::str::contains("::error"));
}

#[test]
fn test_main_verbose_mode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "content\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--verbose").arg(&file_path).assert().success();
}

#[test]
fn test_main_progress_bar_threshold() {
    // Create 11 files to trigger progress bar (threshold is > 10)
    let temp_dir = TempDir::new().unwrap();
    for i in 0..11 {
        let file_path = temp_dir.path().join(format!("file{i}.txt"));
        std::fs::write(&file_path, "content\n").unwrap();
    }

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(temp_dir.path()).assert().success();
}

#[test]
#[cfg(unix)]
fn test_main_fix_with_errors() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("readonly.txt");
    std::fs::write(&file_path, "content").unwrap(); // Missing newline

    // Make file read-only to cause fix error
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o444);
    std::fs::set_permissions(&file_path, perms).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--fix").arg(&file_path).assert().failure().code(1);
}

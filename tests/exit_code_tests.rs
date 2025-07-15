use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_exit_code_0_success() {
    let temp_dir = TempDir::new().unwrap();

    // Create files without issues
    std::fs::write(temp_dir.path().join("good1.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("good2.txt"), "more content\n").unwrap();

    // Test single file
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path().join("good1.txt"))
        .assert()
        .success()
        .code(0);

    // Test multiple files
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path()).assert().success().code(0);

    // Test with --fix on already good files
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--fix")
        .arg(temp_dir.path())
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_exit_code_1_issues_found() {
    let temp_dir = TempDir::new().unwrap();

    // Create files with issues
    std::fs::write(temp_dir.path().join("bad1.txt"), "no newline").unwrap();
    std::fs::write(temp_dir.path().join("bad2.txt"), "trailing spaces  \n").unwrap();

    // Test single file with issue
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path().join("bad1.txt"))
        .assert()
        .failure()
        .code(1);

    // Test multiple files with issues
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path()).assert().failure().code(1);

    // Test mix of good and bad files
    std::fs::write(temp_dir.path().join("good.txt"), "good content\n").unwrap();
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(temp_dir.path()).assert().failure().code(1);
}

#[test]
fn test_exit_code_2_invalid_arguments() {
    // Test with conflicting arguments
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--format")
        .arg("invalid_format")
        .assert()
        .failure()
        .code(2);

    // Test with invalid option
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--invalid-option").assert().failure().code(2);

    // Test with missing required argument value
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--format").assert().failure().code(2);
}

#[test]
fn test_exit_code_3_file_io_error() {
    // Test with non-existent file
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("/path/to/nonexistent/file.txt")
        .assert()
        .success()  // No files found is not an error
        .code(0)
        .stderr(predicate::str::contains("No files found"));

    // Test with git range in non-git directory
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("--from")
        .arg("HEAD~1")
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_exit_code_4_configuration_error() {
    let temp_dir = TempDir::new().unwrap();

    // Test with invalid config file
    let config_path = temp_dir.path().join("bad_config.toml");
    std::fs::write(&config_path, "invalid toml [[[").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--config")
        .arg(&config_path)
        .arg(".")
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("Error loading configuration"));

    // Test with non-existent config file
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--config")
        .arg("/path/to/nonexistent/config.toml")
        .arg(".")
        .assert()
        .failure()
        .code(4);
}

#[test]
fn test_exit_code_1_fix_mode_with_errors() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("readonly.txt");

    // Create file with issues
    std::fs::write(&file_path, "content without newline").unwrap();

    // Make file read-only to cause fix error
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o444);
        std::fs::set_permissions(&file_path, perms).unwrap();
    }

    #[cfg(windows)]
    {
        let mut perms = std::fs::metadata(&file_path).unwrap().permissions();
        perms.set_readonly(true);
        std::fs::set_permissions(&file_path, perms).unwrap();
    }

    // Try to fix read-only file
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--fix").arg(&file_path).assert().failure().code(1);
}

#[test]
#[cfg(unix)]
fn test_permission_errors_still_exit_0() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("unreadable.txt");

    // Create file and make it unreadable
    std::fs::write(&file_path, "content\n").unwrap();
    let mut perms = std::fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o000);
    std::fs::set_permissions(&file_path, perms).unwrap();

    // Permission error should not cause exit code 1
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(&file_path).assert().success().code(0);

    // Restore permissions for cleanup
    let mut perms = std::fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o644);
    std::fs::set_permissions(&file_path, perms).unwrap();
}

#[test]
fn test_stdin_mode_exit_codes() {
    use std::io::Write;
    use std::process::{Command, Stdio};

    // Test stdin with good content (exit 0)
    let mut child = Command::new(assert_cmd::cargo::cargo_bin("lineguard"))
        .arg("--stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"file.txt").unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn test_dry_run_exit_codes() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("fix_me.txt");

    // Create file with issues
    std::fs::write(&file_path, "content  ").unwrap(); // Trailing spaces, no newline

    // Dry run should still exit with 0 (no actual changes made)
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--fix")
        .arg("--dry-run")
        .arg(&file_path)
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Would fix"));
}

#[test]
fn test_quiet_mode_exit_codes() {
    let temp_dir = TempDir::new().unwrap();

    // Create file with issues
    std::fs::write(temp_dir.path().join("bad.txt"), "no newline").unwrap();

    // Quiet mode should suppress output but not affect exit codes
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--quiet")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(1);
    // Note: quiet mode may still show some output in case of issues
}

#[test]
fn test_git_range_exit_codes() {
    // Test with invalid git range (should exit with 3)
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--from")
        .arg("invalid_commit_ref")
        .assert()
        .failure()
        .code(3);
}

#[test]
fn test_multiple_error_conditions() {
    let temp_dir = TempDir::new().unwrap();

    // Create both config error and file with issues
    let config_path = temp_dir.path().join("bad.toml");
    std::fs::write(&config_path, "invalid [[[").unwrap();
    std::fs::write(temp_dir.path().join("bad.txt"), "no newline").unwrap();

    // Config error should take precedence (exit 4)
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--config")
        .arg(&config_path)
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(4);
}

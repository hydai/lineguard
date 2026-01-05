use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_exit_code_0_success() {
    let temp_dir = TempDir::new().unwrap();

    // Create realistic files without issues
    std::fs::write(
        temp_dir.path().join("README.md"),
        "# Project Title\n\nThis is a sample README file.\n",
    )
    .unwrap();
    std::fs::write(
        temp_dir.path().join("config.json"),
        "{\n  \"setting\": \"value\"\n}\n",
    )
    .unwrap();
    std::fs::write(
        temp_dir.path().join("script.py"),
        "#!/usr/bin/env python3\nprint('Hello, World!')\n",
    )
    .unwrap();

    // Test single file
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(temp_dir.path().join("README.md"))
        .assert()
        .success()
        .code(0);

    // Test multiple files via directory
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(temp_dir.path()).assert().success().code(0);

    // Test with --fix on already good files (should remain exit 0)
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--fix")
        .arg(temp_dir.path())
        .assert()
        .success()
        .code(0);

    // Test with specific file extensions
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--extensions")
        .arg("md,json")
        .arg(temp_dir.path())
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_exit_code_1_issues_found() {
    let temp_dir = TempDir::new().unwrap();

    // Create realistic files with common issues
    std::fs::write(
        temp_dir.path().join("README.md"),
        "# Project\n\nDescription without final newline",
    )
    .unwrap();
    std::fs::write(
        temp_dir.path().join("config.yaml"),
        "database:\n  host: localhost  \n  port: 5432\n",
    )
    .unwrap(); // trailing spaces
    std::fs::write(
        temp_dir.path().join("script.sh"),
        "#!/bin/bash\necho 'Hello World'",
    )
    .unwrap(); // missing newline

    // Test single file with missing newline issue
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(temp_dir.path().join("README.md"))
        .assert()
        .failure()
        .code(1);

    // Test single file with trailing spaces issue
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(temp_dir.path().join("config.yaml"))
        .assert()
        .failure()
        .code(1);

    // Test multiple files with various issues
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(temp_dir.path()).assert().failure().code(1);

    // Test mix of good and bad files - should still exit 1 due to issues
    std::fs::write(
        temp_dir.path().join("good_file.txt"),
        "This file is properly formatted.\n",
    )
    .unwrap();
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(temp_dir.path()).assert().failure().code(1);

    // Test with specific extensions that include problematic files
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--extensions")
        .arg("md,yaml")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(1);
}

#[test]
fn test_exit_code_2_invalid_arguments() {
    // Test with conflicting arguments
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--format")
        .arg("invalid_format")
        .assert()
        .failure()
        .code(2);

    // Test with invalid option
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--invalid-option").assert().failure().code(2);

    // Test with missing required argument value
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--format").assert().failure().code(2);
}

#[test]
fn test_exit_code_3_file_io_error() {
    let temp_dir = TempDir::new().unwrap();

    // Test with non-existent file pattern - this should exit 0 with "No files found"
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--quiet")
        .arg(temp_dir.path().join("nonexistent_pattern_*.txt"))
        .assert()
        .success()
        .code(0);

    // Test with git range in non-git directory - this should cause IO error (exit 3)
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(temp_dir.path())
        .arg("--from")
        .arg("HEAD~1")
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("Error"));

    // Test with invalid git commit reference - this should also cause IO error (exit 3)
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(temp_dir.path())
        .arg("--from")
        .arg("nonexistent_commit_hash_12345")
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_exit_code_4_configuration_error() {
    let temp_dir = TempDir::new().unwrap();

    // Create a test file to check (so we're not just testing empty directory)
    std::fs::write(temp_dir.path().join("test.txt"), "content\n").unwrap();

    // Test with invalid TOML syntax
    let config_path = temp_dir.path().join("invalid_syntax.toml");
    std::fs::write(&config_path, "invalid toml syntax [[[").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--config")
        .arg(&config_path)
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("Error loading configuration"));

    // Test with malformed TOML structure
    let malformed_config = temp_dir.path().join("malformed.toml");
    std::fs::write(
        &malformed_config,
        "[section\nmissing_closing_bracket = true",
    )
    .unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--config")
        .arg(&malformed_config)
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("Error loading configuration"));

    // Test with non-existent config file
    let non_existent_config = temp_dir.path().join("does_not_exist.toml");
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--config")
        .arg(&non_existent_config)
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("Error loading configuration"));
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
    let mut cmd = cargo_bin_cmd!("lineguard");
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
    let mut cmd = cargo_bin_cmd!("lineguard");
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

    let temp_dir = TempDir::new().unwrap();

    // Test stdin with good files (exit 0)
    let good_file1 = temp_dir.path().join("good1.txt");
    let good_file2 = temp_dir.path().join("good2.txt");
    std::fs::write(&good_file1, "content with newline\n").unwrap();
    std::fs::write(&good_file2, "more content with newline\n").unwrap();

    let mut child = Command::new(assert_cmd::cargo::cargo_bin!("lineguard"))
        .arg("--stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let mut stdin = child.stdin.take().unwrap();
        let input = format!("{}\n{}\n", good_file1.display(), good_file2.display());
        stdin.write_all(input.as_bytes()).unwrap();
    } // stdin is dropped here, closing the pipe

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0));

    // Test stdin with files that have issues (exit 1)
    let bad_file1 = temp_dir.path().join("bad1.txt");
    let bad_file2 = temp_dir.path().join("bad2.txt");
    std::fs::write(&bad_file1, "no newline").unwrap();
    std::fs::write(&bad_file2, "trailing spaces  \n").unwrap();

    let mut child = Command::new(assert_cmd::cargo::cargo_bin!("lineguard"))
        .arg("--stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let mut stdin = child.stdin.take().unwrap();
        let input = format!("{}\n{}\n", bad_file1.display(), bad_file2.display());
        stdin.write_all(input.as_bytes()).unwrap();
    } // stdin is dropped here, closing the pipe

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(1));

    // Test stdin with mix of good and bad files (exit 1)
    let mut child = Command::new(assert_cmd::cargo::cargo_bin!("lineguard"))
        .arg("--stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let mut stdin = child.stdin.take().unwrap();
        let input = format!("{}\n{}\n", good_file1.display(), bad_file1.display());
        stdin.write_all(input.as_bytes()).unwrap();
    } // stdin is dropped here, closing the pipe

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_dry_run_exit_codes() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("fix_me.txt");

    // Create file with issues
    std::fs::write(&file_path, "content  ").unwrap(); // Trailing spaces, no newline

    // Dry run should still exit with 0 (no actual changes made)
    let mut cmd = cargo_bin_cmd!("lineguard");
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
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--quiet")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(1);
    // Note: quiet mode may still show some output in case of issues
}

#[test]
fn test_git_range_exit_codes() {
    let temp_dir = TempDir::new().unwrap();

    // Test with invalid git range in non-git directory (should exit with 3)
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(temp_dir.path())
        .arg("--from")
        .arg("HEAD~1")
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("Error"));

    // Test with invalid commit reference in non-git directory (should exit with 3)
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(temp_dir.path())
        .arg("--from")
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
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--config")
        .arg(&config_path)
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(4);
}

#[test]
fn test_realistic_project_scenarios() {
    let temp_dir = TempDir::new().unwrap();

    // Create a realistic project structure with mixed file types
    let src_dir = temp_dir.path().join("src");
    let docs_dir = temp_dir.path().join("docs");
    std::fs::create_dir(&src_dir).unwrap();
    std::fs::create_dir(&docs_dir).unwrap();

    // Good files
    std::fs::write(
        src_dir.join("main.rs"),
        "fn main() {\n    println!(\"Hello, world!\");\n}\n",
    )
    .unwrap();
    std::fs::write(
        temp_dir.path().join("Cargo.toml"),
        "[package]\nname = \"test\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    // Files with issues - make sure they have common extensions that will be checked
    std::fs::write(
        docs_dir.join("README.md"),
        "# Project\n\nDocumentation without newline",
    )
    .unwrap();
    std::fs::write(
        src_dir.join("lib.rs"),
        "pub fn hello() {  \n    println!(\"Hello\");\n}",
    )
    .unwrap(); // trailing spaces
    std::fs::write(
        temp_dir.path().join("config.txt"),
        "configuration without newline",
    )
    .unwrap(); // missing newline

    // Test that mixed project exits with code 1 due to issues (check recursively)
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--recursive")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(1);

    // Test that checking only good files exits with code 0
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--extensions")
        .arg("toml")
        .arg(temp_dir.path())
        .assert()
        .success()
        .code(0);

    // Test specific file with issue
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg(temp_dir.path().join("config.txt"))
        .assert()
        .failure()
        .code(1);
}

#[test]
fn test_fix_mode_realistic_scenarios() {
    let temp_dir = TempDir::new().unwrap();

    // Create files that can be successfully fixed
    std::fs::write(
        temp_dir.path().join("fixable1.txt"),
        "content without newline",
    )
    .unwrap();
    std::fs::write(
        temp_dir.path().join("fixable2.py"),
        "#!/usr/bin/env python3\nprint('test')  \n",
    )
    .unwrap(); // trailing spaces

    // Test successful fix should exit with code 0
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--fix")
        .arg(temp_dir.path())
        .assert()
        .success()
        .code(0);

    // Verify files were actually fixed
    let content1 = std::fs::read_to_string(temp_dir.path().join("fixable1.txt")).unwrap();
    assert_eq!(content1, "content without newline\n");

    let content2 = std::fs::read_to_string(temp_dir.path().join("fixable2.py")).unwrap();
    assert_eq!(content2, "#!/usr/bin/env python3\nprint('test')\n");
}

#[test]
fn test_extension_filtering_exit_codes() {
    let temp_dir = TempDir::new().unwrap();

    // Create files with different extensions
    std::fs::write(temp_dir.path().join("good.txt"), "good content\n").unwrap();
    std::fs::write(temp_dir.path().join("bad.txt"), "bad content").unwrap(); // missing newline
    std::fs::write(temp_dir.path().join("good.md"), "# Good markdown\n").unwrap();
    std::fs::write(temp_dir.path().join("bad.md"), "# Bad markdown").unwrap(); // missing newline

    // Test filtering to only good extensions - should exit 0
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--extensions")
        .arg("txt")
        .arg(temp_dir.path().join("good.txt"))
        .assert()
        .success()
        .code(0);

    // Test filtering to only bad extensions - should exit 1
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--extensions")
        .arg("md")
        .arg(temp_dir.path().join("bad.md"))
        .assert()
        .failure()
        .code(1);

    // Test filtering to mixed extensions - should exit 1 due to bad files
    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.arg("--extensions")
        .arg("txt,md")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(1);
}

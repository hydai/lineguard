use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_hidden_files_are_checked_by_default() {
    let temp_dir = TempDir::new().unwrap();

    // Create a hidden file with issues
    std::fs::write(
        temp_dir.path().join(".github-workflow.yml"),
        "line 1  \nline 2",
    )
    .unwrap();

    // Create a normal file without issues
    std::fs::write(temp_dir.path().join("normal.txt"), "line 1\nline 2\n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg(".");

    // Should find issues in the hidden file
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains(".github-workflow.yml"));
}

#[test]
fn test_no_hidden_flag_skips_hidden_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create a hidden file with issues
    std::fs::write(
        temp_dir.path().join(".github-workflow.yml"),
        "line 1  \nline 2",
    )
    .unwrap();

    // Create a normal file with issues
    std::fs::write(temp_dir.path().join("normal.txt"), "line 1  \nline 2").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--no-hidden");
    cmd.arg(".");

    // Should only find issues in normal file, not hidden file
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("normal.txt"))
        .stdout(predicate::str::contains(".github-workflow.yml").not());
}

#[test]
fn test_hidden_directories_are_checked_by_default() {
    let temp_dir = TempDir::new().unwrap();

    // Create .github directory with a file that has issues
    let github_dir = temp_dir.path().join(".github");
    std::fs::create_dir(&github_dir).unwrap();
    std::fs::write(github_dir.join("workflow.yml"), "line 1  \nline 2").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--recursive");
    cmd.arg(".");

    // Should find issues in files within hidden directories
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("workflow.yml"));
}

#[test]
fn test_no_hidden_flag_with_direct_path_still_checks() {
    let temp_dir = TempDir::new().unwrap();

    // Create a hidden file with issues
    let hidden_file = temp_dir.path().join(".hidden.txt");
    std::fs::write(&hidden_file, "line 1  \nline 2").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--no-hidden");
    cmd.arg(hidden_file.to_str().unwrap());

    // Should still check the file when directly specified, even with --no-hidden
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains(".hidden.txt"));
}

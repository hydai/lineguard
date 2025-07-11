use assert_cmd::Command;
use predicates::prelude::*;
use std::process::Command as StdCommand;
use tempfile::TempDir;

fn init_git_repo(dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .args(&["init"])
        .current_dir(repo_path)
        .output()?;

    // Configure git
    StdCommand::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()?;

    StdCommand::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()?;

    Ok(())
}

fn create_commit(
    dir: &TempDir,
    files: &[(&str, &str)],
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let repo_path = dir.path();

    // Create/modify files
    for (filename, content) in files {
        std::fs::write(repo_path.join(filename), content)?;
    }

    // Stage all files
    StdCommand::new("git")
        .args(&["add", "-A"])
        .current_dir(repo_path)
        .output()?;

    // Commit
    StdCommand::new("git")
        .args(&["commit", "-m", message])
        .current_dir(repo_path)
        .output()?;

    // Get commit hash
    let output = StdCommand::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_path)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[test]
fn test_from_option_checks_only_changed_files() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(&temp_dir).unwrap();

    // First commit: create good files
    create_commit(
        &temp_dir,
        &[
            ("file1.txt", "line 1\nline 2\n"),
            ("file2.txt", "line 1\nline 2\n"),
        ],
        "Initial commit",
    )
    .unwrap();

    let first_commit =
        create_commit(&temp_dir, &[("file3.txt", "line 1\nline 2\n")], "Add file3").unwrap();

    // Second commit: add file with issues
    create_commit(
        &temp_dir,
        &[
            ("file4.txt", "line 1  \nline 2"), // Has issues
        ],
        "Add file4 with issues",
    )
    .unwrap();

    // Run lineguard from first commit
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--from").arg(&first_commit);
    cmd.arg(".");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("file4.txt"))
        .stdout(predicate::str::contains("file1.txt").not())
        .stdout(predicate::str::contains("file2.txt").not())
        .stdout(predicate::str::contains("file3.txt").not());
}

#[test]
fn test_from_to_option_checks_range() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(&temp_dir).unwrap();

    // Create commits
    let commit1 =
        create_commit(&temp_dir, &[("file1.txt", "line 1\nline 2\n")], "Commit 1").unwrap();

    let _commit2 = create_commit(
        &temp_dir,
        &[
            ("file2.txt", "line 1  \nline 2\n"), // Has issues
        ],
        "Commit 2",
    )
    .unwrap();

    let commit3 = create_commit(
        &temp_dir,
        &[
            ("file3.txt", "line 1\nline 2"), // Has issues
        ],
        "Commit 3",
    )
    .unwrap();

    create_commit(
        &temp_dir,
        &[
            ("file4.txt", "line 1  \n"), // Has issues
        ],
        "Commit 4",
    )
    .unwrap();

    // Check only commits 2 and 3 (not 4)
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--from").arg(&commit1);
    cmd.arg("--to").arg(&commit3);
    cmd.arg(".");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("file2.txt"))
        .stdout(predicate::str::contains("file3.txt"))
        .stdout(predicate::str::contains("file4.txt").not());
}

#[test]
fn test_from_without_git_repo_shows_error() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file without git repo
    std::fs::write(temp_dir.path().join("file.txt"), "content\n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--from").arg("HEAD~1");
    cmd.arg(".");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not a git repository"));
}

#[test]
fn test_invalid_commit_hash_shows_error() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(&temp_dir).unwrap();

    create_commit(&temp_dir, &[("file.txt", "content\n")], "Initial commit").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--from").arg("invalid-hash");
    cmd.arg(".");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid commit"));
}

#[test]
fn test_from_option_with_fix_mode() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(&temp_dir).unwrap();

    // First commit
    let first_commit = create_commit(
        &temp_dir,
        &[("unchanged.txt", "line 1\nline 2\n")],
        "First commit",
    )
    .unwrap();

    // Second commit with issues
    create_commit(
        &temp_dir,
        &[("changed.txt", "line 1  \nline 2")],
        "Add file with issues",
    )
    .unwrap();

    // Fix only changed files
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--from").arg(&first_commit);
    cmd.arg("--fix");
    cmd.arg(".");

    cmd.assert().success();

    // Verify only changed file was fixed
    let content = std::fs::read_to_string(temp_dir.path().join("changed.txt")).unwrap();
    assert_eq!(content, "line 1\nline 2\n");
}

#[test]
fn test_from_option_with_json_output() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(&temp_dir).unwrap();

    let first_commit =
        create_commit(&temp_dir, &[("file1.txt", "content\n")], "First commit").unwrap();

    create_commit(&temp_dir, &[("file2.txt", "content  \n")], "Second commit").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--from").arg(&first_commit);
    cmd.arg("--format").arg("json");
    cmd.arg(".");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("\"files_checked\": 1"))
        .stdout(predicate::str::contains("file2.txt"));
}

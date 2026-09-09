use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::path::Path;
use std::process::Command as StdCommand;
use tempfile::TempDir;

fn init_git_repo(dir: &Path) {
    StdCommand::new("git")
        .args(["init", "--quiet"])
        .current_dir(dir)
        .output()
        .expect("Failed to initialize git repository");
}

/// A file with a lint issue (trailing space, no trailing newline).
const BAD_CONTENT: &str = "line 1  \nline 2";

#[test]
fn test_respects_gitignore_by_default() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    std::fs::write(temp_dir.path().join(".gitignore"), "ignored.txt\nbuild/\n").unwrap();
    std::fs::write(temp_dir.path().join("ignored.txt"), BAD_CONTENT).unwrap();
    std::fs::write(temp_dir.path().join("checked.txt"), BAD_CONTENT).unwrap();
    std::fs::create_dir(temp_dir.path().join("build")).unwrap();
    std::fs::write(temp_dir.path().join("build/generated.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("-r").arg(".");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("checked.txt"))
        .stdout(predicate::str::contains("ignored.txt").not())
        .stdout(predicate::str::contains("generated.txt").not());
}

#[test]
fn test_no_gitignore_flag_disables_filtering() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    std::fs::write(temp_dir.path().join(".gitignore"), "ignored.txt\n").unwrap();
    std::fs::write(temp_dir.path().join("ignored.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("--no-gitignore").arg("-r").arg(".");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("ignored.txt"));
}

#[test]
fn test_gitignore_ignored_outside_git_repo() {
    // Without a git repository, .gitignore files have no meaning to git,
    // so lineguard does not apply them either.
    let temp_dir = TempDir::new().unwrap();

    std::fs::write(temp_dir.path().join(".gitignore"), "ignored.txt\n").unwrap();
    std::fs::write(temp_dir.path().join("ignored.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("-r").arg(".");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("ignored.txt"));
}

#[test]
fn test_respect_gitignore_false_in_config() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    std::fs::write(
        temp_dir.path().join(".lineguardrc"),
        "respect_gitignore = false\n",
    )
    .unwrap();
    std::fs::write(temp_dir.path().join(".gitignore"), "ignored.txt\n").unwrap();
    std::fs::write(temp_dir.path().join("ignored.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("-r").arg(".");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("ignored.txt"));
}

#[test]
fn test_explicitly_named_file_is_checked_even_if_gitignored() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    std::fs::write(temp_dir.path().join(".gitignore"), "ignored.txt\n").unwrap();
    std::fs::write(temp_dir.path().join("ignored.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("ignored.txt");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("ignored.txt"));
}

#[test]
fn test_nested_gitignore_respected() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    let sub = temp_dir.path().join("sub");
    std::fs::create_dir(&sub).unwrap();
    std::fs::write(sub.join(".gitignore"), "local.txt\n").unwrap();
    std::fs::write(sub.join("local.txt"), BAD_CONTENT).unwrap();
    std::fs::write(sub.join("tracked.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("-r").arg(".");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("tracked.txt"))
        .stdout(predicate::str::contains("local.txt").not());
}

#[test]
fn test_git_dir_contents_are_skipped() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    std::fs::write(temp_dir.path().join(".gitignore"), "ignored.txt\n").unwrap();
    std::fs::write(temp_dir.path().join("ignored.txt"), BAD_CONTENT).unwrap();
    std::fs::write(temp_dir.path().join("checked.txt"), "clean\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("-r").arg(".");

    // Only .gitignore and checked.txt remain: .git internals are not scanned
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"Files checked: 2(\r?\n|$)").unwrap());
}

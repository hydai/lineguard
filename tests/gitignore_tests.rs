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

#[test]
fn test_parent_gitignore_applies_when_scanning_subdirectory() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    // .gitignore at the repository root, scan root is a subdirectory
    std::fs::write(temp_dir.path().join(".gitignore"), "ignored.txt\n").unwrap();
    let sub = temp_dir.path().join("sub");
    std::fs::create_dir(&sub).unwrap();
    std::fs::write(sub.join("ignored.txt"), BAD_CONTENT).unwrap();
    std::fs::write(sub.join("checked.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("-r").arg("sub");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("checked.txt"))
        .stdout(predicate::str::contains("ignored.txt").not());
}

#[test]
fn test_git_info_exclude_is_respected() {
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    std::fs::create_dir_all(temp_dir.path().join(".git/info")).unwrap();
    std::fs::write(temp_dir.path().join(".git/info/exclude"), "excluded.txt\n").unwrap();
    std::fs::write(temp_dir.path().join("excluded.txt"), BAD_CONTENT).unwrap();
    std::fs::write(temp_dir.path().join("checked.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("-r").arg(".");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("checked.txt"))
        .stdout(predicate::str::contains("excluded.txt").not());
}

#[test]
fn test_global_gitignore_is_not_respected() {
    // Results must not depend on per-machine global git configuration.
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    // Fake home directory with a global gitignore via core.excludesFile
    let home = TempDir::new().unwrap();
    let global_ignore = home.path().join("global-ignore");
    std::fs::write(&global_ignore, "globally_ignored.txt\n").unwrap();
    std::fs::write(
        home.path().join(".gitconfig"),
        format!("[core]\n\texcludesFile = {}\n", global_ignore.display()),
    )
    .unwrap();

    std::fs::write(temp_dir.path().join("globally_ignored.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.env("HOME", home.path());
    cmd.env("XDG_CONFIG_HOME", home.path().join(".config"));
    cmd.arg("-r").arg(".");

    // The globally ignored file is still checked and reported
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("globally_ignored.txt"));
}

#[test]
fn test_explicit_git_dir_root_is_scanned() {
    // Explicitly passing .git as the scan root is honored, mirroring
    // explicitly named files: only .git directories encountered while
    // traversing are skipped.
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    std::fs::write(temp_dir.path().join(".git/description"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("-r").arg(".git");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("description"));
}

#[test]
fn test_explicit_gitignored_dir_root_is_scanned() {
    // Same explicit-intent rule for a gitignored directory passed as root
    let temp_dir = TempDir::new().unwrap();
    init_git_repo(temp_dir.path());

    std::fs::write(temp_dir.path().join(".gitignore"), "build/\n").unwrap();
    let build = temp_dir.path().join("build");
    std::fs::create_dir(&build).unwrap();
    std::fs::write(build.join("generated.txt"), BAD_CONTENT).unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("-r").arg("build");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("generated.txt"));
}

#[test]
#[cfg(unix)]
fn test_custom_ignore_pattern_prunes_directory_traversal() {
    use std::os::unix::fs::PermissionsExt;

    // A directory excluded via --ignore must not be traversed at all:
    // walking into it would stat every file and surface errors (like this
    // unreadable directory) from a subtree the user asked to skip.
    let temp_dir = TempDir::new().unwrap();

    let pruned = temp_dir.path().join("pruned");
    std::fs::create_dir(&pruned).unwrap();
    std::fs::write(pruned.join("file.txt"), BAD_CONTENT).unwrap();
    let mut perms = std::fs::metadata(&pruned).unwrap().permissions();
    perms.set_mode(0o000);
    std::fs::set_permissions(&pruned, perms).unwrap();

    std::fs::write(temp_dir.path().join("checked.txt"), "clean\n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("--ignore").arg("pruned").arg("-r").arg(".");

    let assert = cmd.assert().success();

    // Restore permissions so TempDir cleanup works even if assertions fail
    let mut perms = std::fs::metadata(&pruned).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&pruned, perms).unwrap();

    assert
        .stdout(predicate::str::contains("file.txt").not())
        .stderr(predicate::str::contains("pruned").not());
}

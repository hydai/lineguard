use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_ignore_pattern_simple() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("good.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("bad.txt"), "content  \n").unwrap();
    std::fs::write(temp_dir.path().join("ignore.txt"), "content  \n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--ignore").arg("ignore.txt");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .failure()  // Should fail due to bad.txt
        .stdout(predicate::str::contains("\"files_checked\": 2"))  // Only good.txt and bad.txt
        .stdout(predicate::str::contains("bad.txt"))
        .stdout(predicate::str::contains("ignore.txt").not());
}

#[test]
fn test_ignore_pattern_glob() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("test.txt"), "content\n").unwrap();
    std::fs::write(temp_dir.path().join("test.log"), "content  \n").unwrap();
    std::fs::write(temp_dir.path().join("test.tmp"), "content  \n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--ignore").arg("*.log");
    cmd.arg("--ignore").arg("*.tmp");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 1")); // Only test.txt
}

#[test]
fn test_ignore_pattern_directory() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("good.txt"), "content\n").unwrap();
    std::fs::create_dir(temp_dir.path().join("node_modules")).unwrap();
    std::fs::write(temp_dir.path().join("node_modules/bad.txt"), "content  \n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--ignore").arg("**/node_modules/**");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 1")); // Only good.txt
}

#[test]
fn test_ignore_pattern_relative_directory() {
    let temp_dir = TempDir::new().unwrap();

    // Create nested directory structure
    std::fs::create_dir_all(temp_dir.path().join("utils/corpus")).unwrap();
    std::fs::write(temp_dir.path().join("good.txt"), "content\n").unwrap();
    std::fs::write(
        temp_dir.path().join("utils/corpus/ignore_me.txt"),
        "content  \n",
    )
    .unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--ignore").arg("utils/corpus/**");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success() // Should succeed because only good.txt is checked
        .stdout(predicate::str::contains("\"files_checked\": 1"))
        .stdout(predicate::str::contains("ignore_me.txt").not());
}

#[test]
fn test_ignore_pattern_normalization() {
    let temp_dir = TempDir::new().unwrap();

    // Create structure:
    // root/
    //   foo/
    //     bar.txt (bad content)

    std::fs::create_dir_all(temp_dir.path().join("foo")).unwrap();
    let bad_file = temp_dir.path().join("foo/bar.txt");
    std::fs::write(&bad_file, "content  \n").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("--stdin");
    cmd.arg("--ignore").arg("foo/bar.txt");
    cmd.arg("--format").arg("json");

    // We pass path with .. that normalizes to foo/bar.txt
    // foo/../foo/bar.txt -> foo/bar.txt
    cmd.write_stdin("foo/../foo/bar.txt\n");

    // If ignored, no files found -> exit 0, empty stdout (no JSON report), stderr has message
    // If NOT ignored, file found -> checked -> has issues -> exit 1, JSON report with issues
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("No files found to check"));
}

#[test]
fn test_ignore_pattern_redundant_prefixes() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    std::fs::write(temp_dir.path().join("good.txt"), "content\n").unwrap();
    std::fs::create_dir_all(temp_dir.path().join("ignored_dir")).unwrap();
    std::fs::write(
        temp_dir.path().join("ignored_dir/ignore_me.txt"),
        "content  \n",
    )
    .unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg("--stdin");
    cmd.arg("--ignore").arg("ignored_dir/**");
    cmd.arg("--format").arg("json");
    cmd.write_stdin("good.txt\n././ignored_dir/ignore_me.txt\n");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 1")); // Only good.txt
}

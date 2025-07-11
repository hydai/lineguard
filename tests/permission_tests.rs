use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

#[test]
#[cfg(target_family = "unix")]
fn test_unreadable_file_error_message() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("unreadable.txt");

    // Create a file with no read permissions
    fs::write(&file_path, "content\n").unwrap();
    let metadata = fs::metadata(&file_path).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o000); // No permissions
    fs::set_permissions(&file_path, permissions).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("unreadable.txt");

    cmd.assert()
        .success() // Should not fail, just skip the file
        .stderr(predicate::str::contains("Permission denied"))
        .stderr(predicate::str::contains("unreadable.txt"));

    // Clean up - restore permissions for cleanup
    let mut permissions = fs::metadata(&file_path).unwrap().permissions();
    permissions.set_mode(0o644);
    fs::set_permissions(&file_path, permissions).unwrap();
}

#[test]
#[cfg(target_family = "unix")]
fn test_unwritable_file_fix_error() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("readonly.txt");

    // Create a file with trailing spaces
    fs::write(&file_path, "content  \n").unwrap();

    // Make it read-only
    let metadata = fs::metadata(&file_path).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o444); // Read-only
    fs::set_permissions(&file_path, permissions).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("readonly.txt");
    cmd.arg("--fix");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Permission denied"))
        .stderr(predicate::str::contains("readonly.txt"));

    // Clean up - restore permissions
    let mut permissions = fs::metadata(&file_path).unwrap().permissions();
    permissions.set_mode(0o644);
    fs::set_permissions(&file_path, permissions).unwrap();
}

#[test]
#[cfg(target_family = "unix")]
fn test_unreadable_directory_error() {
    let temp_dir = TempDir::new().unwrap();
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();

    // Create a file inside
    fs::write(sub_dir.join("file.txt"), "content\n").unwrap();

    // Make directory unreadable
    let metadata = fs::metadata(&sub_dir).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o000); // No permissions
    fs::set_permissions(&sub_dir, permissions).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("subdir");
    cmd.arg("--recursive");

    cmd.assert()
        .success() // Should handle gracefully
        .stderr(predicate::str::contains("Permission denied"))
        .stderr(predicate::str::contains("subdir"));

    // Clean up - restore permissions
    let mut permissions = fs::metadata(&sub_dir).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&sub_dir, permissions).unwrap();
}

#[test]
#[cfg(target_family = "unix")]
fn test_permission_error_json_format() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("unreadable.txt");

    // Create a file with no read permissions
    fs::write(&file_path, "content\n").unwrap();
    let metadata = fs::metadata(&file_path).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o000); // No permissions
    fs::set_permissions(&file_path, permissions).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("unreadable.txt");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"errors\""))
        .stdout(predicate::str::contains("Permission denied"));

    // Clean up
    let mut permissions = fs::metadata(&file_path).unwrap().permissions();
    permissions.set_mode(0o644);
    fs::set_permissions(&file_path, permissions).unwrap();
}

#[test]
#[cfg(target_family = "unix")]
fn test_multiple_files_with_permission_errors() {
    let temp_dir = TempDir::new().unwrap();

    // Create readable file
    fs::write(temp_dir.path().join("good.txt"), "content\n").unwrap();

    // Create unreadable file
    let unreadable = temp_dir.path().join("bad.txt");
    fs::write(&unreadable, "content  \n").unwrap();
    let metadata = fs::metadata(&unreadable).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&unreadable, permissions).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");

    cmd.assert()
        .success() // Should continue checking other files
        .stderr(predicate::str::contains("Permission denied"))
        .stdout(predicate::str::contains("good.txt").not()); // Good file should pass

    // Clean up
    let mut permissions = fs::metadata(&unreadable).unwrap().permissions();
    permissions.set_mode(0o644);
    fs::set_permissions(&unreadable, permissions).unwrap();
}

// Windows-specific permission tests
#[test]
#[cfg(windows)]
fn test_readonly_file_on_windows() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("readonly.txt");

    // Create a file with trailing spaces
    fs::write(&file_path, "content  \n").unwrap();

    // Make it read-only using Windows-specific permissions
    let mut permissions = fs::metadata(&file_path).unwrap().permissions();
    permissions.set_readonly(true);
    fs::set_permissions(&file_path, permissions).unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("readonly.txt");
    cmd.arg("--fix");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("denied").or(predicate::str::contains("read-only")));

    // Clean up - restore permissions
    let mut permissions = fs::metadata(&file_path).unwrap().permissions();
    permissions.set_readonly(false);
    fs::set_permissions(&file_path, permissions).unwrap();
}

#[test]
#[cfg(windows)]
fn test_permission_error_handling_windows() {
    let temp_dir = TempDir::new().unwrap();
    
    // On Windows, we can test with read-only files
    let readonly_file = temp_dir.path().join("readonly.txt");
    fs::write(&readonly_file, "content\n").unwrap();
    
    // Make file read-only
    let mut permissions = fs::metadata(&readonly_file).unwrap().permissions();
    permissions.set_readonly(true);
    fs::set_permissions(&readonly_file, permissions).unwrap();
    
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("readonly.txt");
    
    // The tool should handle read-only files gracefully
    cmd.assert()
        .success(); // Read-only files can still be read on Windows
    
    // Clean up
    let mut permissions = fs::metadata(&readonly_file).unwrap().permissions();
    permissions.set_readonly(false);
    fs::set_permissions(&readonly_file, permissions).unwrap();
}
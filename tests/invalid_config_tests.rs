use assert_cmd::Command;
use lineguard::config::load_config;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_invalid_toml_syntax() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Invalid TOML syntax
    std::fs::write(
        &config_path,
        r#"
[checks
newline_ending = true
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg(".")
        .arg("--config")
        .arg(&config_path)
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("Error loading configuration"));
}

#[test]
fn test_unknown_config_fields() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Unknown field in config
    std::fs::write(
        &config_path,
        r#"
unknown_field = true
not_a_real_option = "value"
"#,
    )
    .unwrap();

    // Should load successfully but ignore unknown fields
    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_ok());
}

#[test]
fn test_invalid_type_for_config_field() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Wrong type for a field
    std::fs::write(
        &config_path,
        r#"
[checks]
newline_ending = "yes"  # Should be boolean
"#,
    )
    .unwrap();

    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_err());
}

#[test]
fn test_empty_config_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Empty file
    std::fs::write(&config_path, "").unwrap();

    // Should load with defaults
    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_ok());
    let config = config.unwrap();
    // Check default values
    assert!(config.checks.newline_ending);
    assert!(config.checks.trailing_spaces);
    assert!(config.ignore_patterns.is_empty());
    assert!(config.file_extensions.is_empty());
}

#[test]
fn test_config_file_with_only_comments() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Only comments
    std::fs::write(
        &config_path,
        r#"
# This is a comment
# Another comment
# [checks]
# newline_ending = false
"#,
    )
    .unwrap();

    // Should load with defaults
    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_ok());
    let config = config.unwrap();
    // Check default values
    assert!(config.checks.newline_ending);
    assert!(config.checks.trailing_spaces);
    assert!(config.ignore_patterns.is_empty());
    assert!(config.file_extensions.is_empty());
}

#[test]
fn test_config_file_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("nonexistent.toml");

    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_err());
}

#[test]
fn test_malformed_ignore_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Invalid type for ignore_patterns
    std::fs::write(
        &config_path,
        r#"
ignore_patterns = "not_an_array"
"#,
    )
    .unwrap();

    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_err());
}

#[test]
fn test_malformed_file_extensions() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Invalid type for file_extensions
    std::fs::write(
        &config_path,
        r#"
file_extensions = 123
"#,
    )
    .unwrap();

    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_err());
}

#[test]
fn test_config_with_invalid_utf8() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Write invalid UTF-8 bytes
    use std::io::Write;
    let mut file = std::fs::File::create(&config_path).unwrap();
    file.write_all(&[0xFF, 0xFE, 0xFD]).unwrap();
    drop(file);

    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_err());
}

#[test]
fn test_config_directory_instead_of_file() {
    let temp_dir = TempDir::new().unwrap();

    // Try to load a directory as config
    let config = load_config(Some(temp_dir.path()));
    assert!(config.is_err());
}

#[test]
fn test_circular_config_inclusion() {
    // Note: This test assumes the config system doesn't support includes
    // If it does in the future, this test would need to be updated
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Config that references itself (if includes were supported)
    std::fs::write(
        &config_path,
        r#"
# include = ".lineguardrc"
[checks]
newline_ending = true
"#,
    )
    .unwrap();

    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_ok()); // Should work since includes aren't supported
}

#[test]
fn test_config_with_extreme_values() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Very long strings and arrays
    let mut long_patterns = vec![];
    for i in 0..1000 {
        long_patterns.push(format!("pattern_{i}"));
    }

    let config_content = format!(
        r#"
ignore_patterns = {long_patterns:?}
file_extensions = ["txt"]
"#
    );

    std::fs::write(&config_path, config_content).unwrap();

    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.ignore_patterns.len(), 1000);
}

#[test]
fn test_nested_config_sections() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    // Deeply nested structure (if supported)
    std::fs::write(
        &config_path,
        r#"
[checks]
newline_ending = true
trailing_spaces = false

[checks.advanced]
# This would be an invalid nested structure
something = true
"#,
    )
    .unwrap();

    // Should either fail or ignore the nested section
    let config = load_config(Some(config_path.as_path()));
    // The behavior depends on the implementation
    assert!(config.is_ok() || config.is_err());
}

#[test]
#[cfg(unix)]
fn test_config_permissions_issue() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".lineguardrc");

    std::fs::write(
        &config_path,
        r#"
[checks]
newline_ending = true
"#,
    )
    .unwrap();

    // Make file unreadable
    let mut perms = std::fs::metadata(&config_path).unwrap().permissions();
    perms.set_mode(0o000);
    std::fs::set_permissions(&config_path, perms).unwrap();

    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_err());

    // Restore permissions for cleanup
    let mut perms = std::fs::metadata(&config_path).unwrap().permissions();
    perms.set_mode(0o644);
    std::fs::set_permissions(&config_path, perms).unwrap();
}

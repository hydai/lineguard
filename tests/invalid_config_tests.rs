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
    // Unknown fields should be ignored by serde's default behavior
    let config = load_config(Some(config_path.as_path()));
    assert!(config.is_ok());

    // Verify that the configuration loads successfully with default values
    let config = config.unwrap();
    assert!(config.checks.newline_ending); // Default value
    assert!(config.checks.trailing_spaces); // Default value
    assert!(config.ignore_patterns.is_empty()); // Default value
    assert!(config.file_extensions.is_empty()); // Default value
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

    // Verify that the error message contains information about the type mismatch
    let error = config.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("invalid type")
            || error_msg.contains("expected")
            || error_msg.contains("boolean")
    );
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

    // Verify that the error message indicates the file was not found
    let error = config.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("Configuration file not found")
            && error_msg.contains("nonexistent.toml")
    );
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

    // Verify that the error message indicates a type mismatch for ignore_patterns
    let error = config.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("invalid type")
            || error_msg.contains("expected")
            || error_msg.contains("array")
            || error_msg.contains("sequence")
    );
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

    // Verify that the error message indicates a type mismatch for file_extensions
    let error = config.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("invalid type")
            || error_msg.contains("expected")
            || error_msg.contains("array")
            || error_msg.contains("sequence")
    );
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

    // Verify that the error message indicates UTF-8 encoding issues
    let error = config.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("UTF-8")
            || error_msg.contains("utf8")
            || error_msg.contains("invalid")
            || error_msg.contains("encoding")
    );
}

#[test]
fn test_config_directory_instead_of_file() {
    let temp_dir = TempDir::new().unwrap();

    // Try to load a directory as config
    let config = load_config(Some(temp_dir.path()));
    assert!(config.is_err());

    // Verify that the error message indicates an I/O error (directory cannot be read as file)
    let error = config.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("Is a directory")
            || error_msg.contains("directory")
            || error_msg.contains("I/O")
    );
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
    // Nested sections should be ignored by serde, so config should load successfully
    assert!(config.is_ok());

    // Verify that the valid configuration values are loaded correctly
    let config = config.unwrap();
    assert!(config.checks.newline_ending); // Should be true as specified
    assert!(!config.checks.trailing_spaces); // Should be false as specified
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

    // Verify that the error message indicates permission issues
    let error = config.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("Permission denied")
            || error_msg.contains("permission")
            || error_msg.contains("denied")
    );

    // Restore permissions for cleanup
    let mut perms = std::fs::metadata(&config_path).unwrap().permissions();
    perms.set_mode(0o644);
    std::fs::set_permissions(&config_path, perms).unwrap();
}

use lineguard::config::Config;
use lineguard::discovery::should_check_file;
use std::path::Path;

#[test]
fn test_should_check_regular_file() {
    let config = Config::default();
    let path = Path::new("test.txt");

    assert!(should_check_file(path, &config));
}

#[test]
fn test_should_check_with_extensions_filter() {
    let config = Config {
        file_extensions: vec!["rs".to_string(), "txt".to_string()],
        ..Default::default()
    };

    assert!(should_check_file(Path::new("test.rs"), &config));
    assert!(should_check_file(Path::new("file.txt"), &config));
    assert!(!should_check_file(Path::new("image.png"), &config));
    assert!(!should_check_file(Path::new("data.json"), &config));
}

#[test]
fn test_should_skip_hidden_files() {
    let config = Config::default();

    assert!(!should_check_file(Path::new(".gitignore"), &config));
    assert!(!should_check_file(Path::new(".hidden"), &config));
    assert!(should_check_file(Path::new("visible.txt"), &config));
}

#[test]
fn test_should_skip_common_binary_extensions() {
    let config = Config::default();

    // Binary files
    assert!(!should_check_file(Path::new("image.jpg"), &config));
    assert!(!should_check_file(Path::new("program.exe"), &config));
    assert!(!should_check_file(Path::new("archive.zip"), &config));
    assert!(!should_check_file(Path::new("data.bin"), &config));

    // Text files
    assert!(should_check_file(Path::new("code.rs"), &config));
    assert!(should_check_file(Path::new("readme.md"), &config));
    assert!(should_check_file(Path::new("config.toml"), &config));
}

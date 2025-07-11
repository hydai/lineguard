use lineguard::cli::CliArgs;
use lineguard::config::Config;
use lineguard::discovery::discover_files;
use tempfile::TempDir;

#[test]
fn test_discover_single_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "test content").unwrap();

    let args = CliArgs {
        files: vec![file_path.to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0], file_path);
}

#[test]
fn test_discover_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    let file3 = temp_dir.path().join("file3.txt");

    std::fs::write(&file1, "content1").unwrap();
    std::fs::write(&file2, "content2").unwrap();
    std::fs::write(&file3, "content3").unwrap();

    let args = CliArgs {
        files: vec![
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
            file3.to_string_lossy().to_string(),
        ],
        stdin: false,
        recursive: false,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();
    assert_eq!(result.files.len(), 3);
    assert!(result.files.contains(&file1));
    assert!(result.files.contains(&file2));
    assert!(result.files.contains(&file3));
}

#[test]
fn test_discover_files_with_glob_pattern() {
    let temp_dir = TempDir::new().unwrap();
    let txt1 = temp_dir.path().join("file1.txt");
    let txt2 = temp_dir.path().join("file2.txt");
    let rs1 = temp_dir.path().join("file1.rs");

    std::fs::write(&txt1, "content").unwrap();
    std::fs::write(&txt2, "content").unwrap();
    std::fs::write(&rs1, "content").unwrap();

    let glob_pattern = format!("{}/*.txt", temp_dir.path().display());
    let args = CliArgs {
        files: vec![glob_pattern],
        stdin: false,
        recursive: false,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();
    assert_eq!(result.files.len(), 2);
    assert!(
        result
            .files
            .iter()
            .any(|p| p.file_name().unwrap() == "file1.txt")
    );
    assert!(
        result
            .files
            .iter()
            .any(|p| p.file_name().unwrap() == "file2.txt")
    );
    assert!(
        !result
            .files
            .iter()
            .any(|p| p.file_name().unwrap() == "file1.rs")
    );
}

#[test]
fn test_discover_files_in_directory() {
    let temp_dir = TempDir::new().unwrap();
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.rs");

    std::fs::write(&file1, "content").unwrap();
    std::fs::write(&file2, "content").unwrap();

    let args = CliArgs {
        files: vec![temp_dir.path().to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();
    assert_eq!(result.files.len(), 2);
    assert!(result.files.contains(&file1));
    assert!(result.files.contains(&file2));
}

#[test]
fn test_discover_files_recursive() {
    let temp_dir = TempDir::new().unwrap();
    let subdir = temp_dir.path().join("subdir");
    std::fs::create_dir(&subdir).unwrap();

    let file1 = temp_dir.path().join("file1.txt");
    let file2 = subdir.join("file2.txt");
    let file3 = subdir.join("file3.rs");

    std::fs::write(&file1, "content").unwrap();
    std::fs::write(&file2, "content").unwrap();
    std::fs::write(&file3, "content").unwrap();

    let args = CliArgs {
        files: vec![temp_dir.path().to_string_lossy().to_string()],
        stdin: false,
        recursive: true,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();
    assert_eq!(result.files.len(), 3);
    assert!(result.files.contains(&file1));
    assert!(result.files.contains(&file2));
    assert!(result.files.contains(&file3));
}

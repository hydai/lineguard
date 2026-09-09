use lineguard::checker::check_file;
use lineguard::cli::CliArgs;
use lineguard::config::Config;
use lineguard::discovery::discover_files;
use std::sync::Arc;
use tempfile::TempDir;

#[test]
fn test_unicode_file_names() {
    let temp_dir = TempDir::new().unwrap();

    // Various Unicode file names
    let unicode_files = vec![
        "测试文件.txt",      // Chinese
        "тест.txt",          // Russian
        "テスト.txt",        // Japanese
        "파일.txt",          // Korean
        "αρχείο.txt",        // Greek
        "ملف.txt",           // Arabic
        "fichier_été.txt",   // French with accents
        "arquivo_ção.txt",   // Portuguese
        "datei_äöü.txt",     // German
        "emoji_🚀_file.txt", // Emoji
    ];

    // Create files with Unicode names
    for filename in &unicode_files {
        let file_path = temp_dir.path().join(filename);
        std::fs::write(&file_path, "content\n").unwrap();
    }

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
        no_hidden: false,
        no_gitignore: false,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();

    // Should discover all Unicode files
    assert_eq!(result.files.len(), unicode_files.len());

    // Check that all files can be processed
    let config_arc = Arc::new(config);
    for file_path in result.files {
        let check_result = check_file(&file_path, &config_arc);
        assert!(check_result.error.is_none());
        assert!(check_result.issues.is_empty());
    }
}

#[test]
fn test_unicode_directory_names() {
    let temp_dir = TempDir::new().unwrap();

    // Create directories with Unicode names
    let unicode_dirs = vec![
        "目录",        // Chinese "directory"
        "папка",       // Russian "folder"
        "フォルダ",    // Japanese "folder"
        "폴더",        // Korean "folder"
        "dossier_été", // French "summer folder"
    ];

    for dirname in &unicode_dirs {
        let dir_path = temp_dir.path().join(dirname);
        std::fs::create_dir(&dir_path).unwrap();

        // Add a file in each Unicode directory
        let file_path = dir_path.join("test.txt");
        std::fs::write(&file_path, "content\n").unwrap();
    }

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
        no_hidden: false,
        no_gitignore: false,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();

    // Should find one file in each Unicode directory
    assert_eq!(result.files.len(), unicode_dirs.len());
}

#[test]
fn test_unicode_glob_patterns() {
    let temp_dir = TempDir::new().unwrap();

    // Create files with Unicode names
    let files = vec!["测试1.txt", "测试2.txt", "test3.txt", "тест4.txt"];

    for filename in &files {
        let file_path = temp_dir.path().join(filename);
        std::fs::write(&file_path, "content\n").unwrap();
    }

    // Test glob pattern with Unicode prefix
    let glob_pattern = format!("{}/测试*.txt", temp_dir.path().display());

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
        no_hidden: false,
        no_gitignore: false,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();

    // Should find only the Chinese-prefixed files
    assert_eq!(result.files.len(), 2);
    assert!(
        result
            .files
            .iter()
            .all(|p| p.file_name().unwrap().to_string_lossy().starts_with("测试"))
    );
}

#[test]
fn test_unicode_file_with_issues() {
    let temp_dir = TempDir::new().unwrap();

    // Create file with Unicode name and issues
    let file_path = temp_dir.path().join("файл_с_проблемами.txt");
    std::fs::write(&file_path, "content with trailing space   ").unwrap(); // Missing newline and trailing space

    let config = Arc::new(Config::default());
    let check_result = check_file(&file_path, &config);

    assert!(check_result.error.is_none());
    assert_eq!(check_result.issues.len(), 2); // Should find both issues

    // Verify the file path is correctly preserved
    assert_eq!(check_result.file_path, file_path);
}

#[test]
fn test_unicode_in_ignore_patterns() {
    let temp_dir = TempDir::new().unwrap();

    // Create files
    let files = vec![
        "test.txt",
        "忽略.txt", // "ignore" in Chinese
        "keep.txt",
    ];

    for filename in &files {
        let file_path = temp_dir.path().join(filename);
        std::fs::write(&file_path, "content\n").unwrap();
    }

    let args = CliArgs {
        files: vec![temp_dir.path().to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec!["**/忽略.txt".to_string()],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
        no_hidden: false,
        no_gitignore: false,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();

    // Should ignore the Chinese-named file
    assert_eq!(result.files.len(), 2);
    assert!(
        !result
            .files
            .iter()
            .any(|p| p.file_name().unwrap().to_string_lossy().contains("忽略"))
    );
}

#[test]
fn test_mixed_unicode_and_ascii_paths() {
    let temp_dir = TempDir::new().unwrap();

    // Create a complex directory structure with mixed names
    let structure = vec![
        ("normal_dir", vec!["file1.txt", "файл2.txt"]),
        ("目录_test", vec!["test.txt", "测试.txt"]),
    ];

    for (dir_name, files) in structure {
        let dir_path = temp_dir.path().join(dir_name);
        std::fs::create_dir(&dir_path).unwrap();

        for file_name in files {
            let file_path = dir_path.join(file_name);
            std::fs::write(&file_path, "content\n").unwrap();
        }
    }

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
        no_hidden: false,
        no_gitignore: false,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();

    // Should find all 4 files
    assert_eq!(result.files.len(), 4);

    // All files should be checkable
    let config_arc = Arc::new(config);
    for file_path in result.files {
        let check_result = check_file(&file_path, &config_arc);
        assert!(check_result.error.is_none());
    }
}

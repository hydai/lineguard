use lineguard::checker::IssueType;
use lineguard::checker::check_file;
use lineguard::config::Config;
use lineguard::fixer::fix_file;
use rayon::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_concurrent_file_checking() {
    let temp_dir = TempDir::new().unwrap();
    let config = Arc::new(Config::default());

    // Create 100 files with issues
    let files: Vec<_> = (0..100)
        .map(|i| {
            let path = temp_dir.path().join(format!("file_{i}.txt"));
            std::fs::write(&path, "content without newline").unwrap();
            path
        })
        .collect();

    // Check all files concurrently
    let results: Vec<_> = files
        .par_iter()
        .map(|path| check_file(path, &config))
        .collect();

    // All checks should succeed and find the missing newline issue
    assert_eq!(results.len(), 100);
    for result in results {
        assert!(result.error.is_none());
        assert_eq!(result.issues.len(), 1);
        assert_eq!(result.issues[0].issue_type, IssueType::MissingNewline);
    }
}

#[test]
fn test_concurrent_file_fixing() {
    let temp_dir = TempDir::new().unwrap();
    let config = Arc::new(Config::default());

    // Create files with issues
    let files: Vec<_> = (0..50)
        .map(|i| {
            let path = temp_dir.path().join(format!("fix_{i}.txt"));
            std::fs::write(&path, "content  ").unwrap(); // Trailing spaces and missing newline
            path
        })
        .collect();

    // First check all files to get issues
    let check_results: Vec<_> = files
        .par_iter()
        .map(|path| check_file(path, &config))
        .collect();

    // Fix all files concurrently
    let fix_results: Vec<_> = files
        .par_iter()
        .zip(check_results.par_iter())
        .map(|(path, check_result)| fix_file(path, &check_result.issues, &config, false))
        .collect();

    // All fixes should succeed
    assert_eq!(fix_results.len(), 50);
    for result in fix_results {
        assert!(result.is_ok());
        let fix_result = result.unwrap();
        assert!(fix_result.fixed);
        assert_eq!(fix_result.issues_fixed.len(), 2); // Both trailing space and missing newline
    }

    // Verify files are actually fixed
    for path in &files {
        let content = std::fs::read_to_string(path).unwrap();
        assert_eq!(content, "content\n");
    }
}

#[test]
fn test_file_modification_during_check() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("changing_file.txt");
    let config = Arc::new(Config::default());

    // Create initial file
    std::fs::write(&file_path, "initial content\n").unwrap();

    // Spawn a thread that will modify the file
    let file_path_clone = file_path.clone();
    let modifier_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));

        // Try to modify the file multiple times
        for i in 0..5 {
            if let Ok(mut file) = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&file_path_clone)
            {
                let _ = writeln!(file, "modified content {i}");
            }
            thread::sleep(Duration::from_millis(5));
        }
    });

    // Perform multiple checks while file is being modified
    let mut results = vec![];
    for _ in 0..10 {
        results.push(check_file(&file_path, &config));
        thread::sleep(Duration::from_millis(3));
    }

    modifier_thread.join().unwrap();

    // All checks should complete without panicking
    // Some may have issues, some may not, depending on timing
    assert_eq!(results.len(), 10);
    for result in results {
        assert!(result.error.is_none()); // No errors, just potential issues
    }
}

#[test]
fn test_concurrent_read_write_operations() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("concurrent_rw.txt");
    let config = Arc::new(Config::default());

    // Create initial file with issue
    std::fs::write(&file_path, "content  ").unwrap(); // Trailing spaces

    // Spawn multiple reader threads
    let mut reader_handles = vec![];
    for i in 0..5 {
        let path = file_path.clone();
        let cfg = config.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 5));
            check_file(&path, &cfg)
        });
        reader_handles.push(handle);
    }

    // Spawn a writer thread that fixes the file
    let writer_path = file_path.clone();
    let writer_config = config.clone();
    let writer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(15));
        let check_result = check_file(&writer_path, &writer_config);
        if !check_result.issues.is_empty() {
            fix_file(&writer_path, &check_result.issues, &writer_config, false)
        } else {
            Ok(lineguard::fixer::FixResult {
                file_path: writer_path,
                fixed: false,
                issues_fixed: vec![],
            })
        }
    });

    // Collect all results
    let mut read_results = vec![];
    for handle in reader_handles {
        read_results.push(handle.join().unwrap());
    }
    let write_result = writer_handle.join().unwrap();

    // All operations should complete successfully
    assert_eq!(read_results.len(), 5);
    for result in read_results {
        assert!(result.error.is_none());
    }
    assert!(write_result.is_ok());
}

#[test]
fn test_large_scale_concurrent_processing() {
    let temp_dir = TempDir::new().unwrap();
    let config = Arc::new(Config::default());

    // Create a mix of good and bad files
    let files: Vec<_> = (0..200)
        .map(|i| {
            let path = temp_dir.path().join(format!("large_scale_{i}.txt"));
            if i % 3 == 0 {
                // File with trailing spaces
                std::fs::write(&path, "content with spaces   \n").unwrap();
            } else if i % 3 == 1 {
                // File missing newline
                std::fs::write(&path, "content without newline").unwrap();
            } else {
                // Good file
                std::fs::write(&path, "good content\n").unwrap();
            }
            path
        })
        .collect();

    // Process all files concurrently
    let results: Vec<_> = files
        .par_iter()
        .map(|path| {
            let check_result = check_file(path, &config);
            if !check_result.issues.is_empty() {
                fix_file(path, &check_result.issues, &config, false)
            } else {
                Ok(lineguard::fixer::FixResult {
                    file_path: path.clone(),
                    fixed: false,
                    issues_fixed: vec![],
                })
            }
        })
        .collect();

    // Count results
    let fixed_count = results
        .iter()
        .filter(|r| r.as_ref().map(|f| f.fixed).unwrap_or(false))
        .count();
    let unchanged_count = results
        .iter()
        .filter(|r| r.as_ref().map(|f| !f.fixed).unwrap_or(false))
        .count();

    assert_eq!(results.len(), 200);
    assert!(fixed_count > 0);
    assert!(unchanged_count > 0);
    assert_eq!(fixed_count + unchanged_count, 200);
}

#[test]
#[cfg(unix)]
fn test_file_locking_behavior() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("locked_file.txt");
    let config = Arc::new(Config::default());

    // Create file with content
    std::fs::write(&file_path, "content  ").unwrap(); // Trailing spaces

    // Create a file handle to keep it open
    let _file_handle = File::open(&file_path).unwrap();

    // Try to check and fix while file is open for reading
    let check_result = check_file(&file_path, &config);
    assert!(check_result.error.is_none());
    assert_eq!(check_result.issues.len(), 2); // Trailing space and missing newline

    // Fixing should still work on Unix (multiple readers allowed)
    let fix_result = fix_file(&file_path, &check_result.issues, &config, false);
    assert!(fix_result.is_ok());
}

#[test]
fn test_concurrent_directory_traversal() {
    let temp_dir = TempDir::new().unwrap();
    let config = Arc::new(Config::default());

    // Create nested directory structure
    for i in 0..10 {
        let subdir = temp_dir.path().join(format!("dir_{i}"));
        std::fs::create_dir(&subdir).unwrap();

        for j in 0..10 {
            let file_path = subdir.join(format!("file_{j}.txt"));
            std::fs::write(&file_path, "content\n").unwrap();
        }
    }

    // Use discover_files with parallel processing
    use lineguard::cli::CliArgs;
    use lineguard::discovery::discover_files;

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
    };

    let result = discover_files(&args, &Config::default()).unwrap();
    assert_eq!(result.files.len(), 100);

    // Check all discovered files concurrently
    let check_results: Vec<_> = result
        .files
        .par_iter()
        .map(|path| check_file(path, &config))
        .collect();

    assert_eq!(check_results.len(), 100);
    for result in check_results {
        assert!(result.error.is_none());
        assert!(result.issues.is_empty()); // All files are good
    }
}

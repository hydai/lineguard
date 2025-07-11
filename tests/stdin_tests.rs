use lineguard::cli::CliArgs;
use lineguard::config::Config;
use lineguard::discovery::discover_files;

#[test]
fn test_discover_files_from_stdin_empty() {
    let args = CliArgs {
        files: vec![],
        stdin: true,
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
    };

    // With stdin flag but no actual stdin, should return empty
    let config = Config::default();
    let result = discover_files(&args, &config);
    assert!(result.is_ok());
}

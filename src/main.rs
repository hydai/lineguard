use indicatif::{ProgressBar, ProgressStyle};
use lineguard::checker::check_file;
use lineguard::cli::{OutputFormat, parse_args};
use lineguard::config::load_config;
use lineguard::discovery::discover_files;
use lineguard::fixer::fix_file;
use lineguard::reporter::{GitHubReporter, HumanReporter, JsonReporter, Reporter};
use rayon::prelude::*;
use std::process;
use std::sync::{Arc, Mutex};

fn main() {
    let args = parse_args();

    // Load configuration
    let mut config = match load_config(args.config.as_deref()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {e}");
            process::exit(4);
        },
    };

    // Apply CLI flags to override config
    if args.no_newline_check {
        config.checks.newline_ending = false;
    }
    if args.no_trailing_space {
        config.checks.trailing_spaces = false;
    }

    // Discover files to check
    let discovery_result = match discover_files(&args, &config) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(3);
        },
    };

    let files = discovery_result.files;

    // Show git range info in verbose mode
    if args.verbose && discovery_result.git_range.is_some() {
        if let Some(git_info) = &discovery_result.git_range {
            println!(
                "Git range: {}..{}",
                &git_info.from[0..7.min(git_info.from.len())],
                &git_info.to[0..7.min(git_info.to.len())]
            );
            println!("Changed files: {}", git_info.changed_files.len());
            if !args.quiet {
                for file in &git_info.changed_files {
                    println!("  - {}", file.display());
                }
                println!();
            }
        }
    }

    if files.is_empty() && !args.quiet {
        eprintln!("No files found to check");
        process::exit(0);
    }

    // Show appropriate message for human format
    if !args.quiet && args.format == OutputFormat::Human && files.len() > 1 {
        if args.fix {
            if args.dry_run {
                println!("Checking {} files (dry run)...", files.len());
            } else {
                println!("Fixing {} files...", files.len());
            }
        } else {
            println!("Checking {} files...", files.len());
        }
    }

    // Set up progress bar for large file sets
    let progress_bar = if files.len() > 10 && !args.quiet && args.format == OutputFormat::Human {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("Checking files... [{bar:40}] {pos}/{len} ({percent}%)")
                .unwrap()
                .progress_chars("████░"),
        );
        Some(pb)
    } else {
        None
    };

    let pb_mutex = progress_bar.as_ref().map(Mutex::new);
    let config_arc = Arc::new(config.clone());
    let fix_mode = args.fix;
    let dry_run = args.dry_run;

    if fix_mode {
        // Fix mode: check and fix files
        let fix_results: Vec<_> = files
            .par_iter()
            .map(|file_path| {
                let check_result = check_file(file_path, &config_arc);
                let fix_result = if !check_result.issues.is_empty() {
                    fix_file(file_path, &check_result.issues, &config_arc, dry_run)
                } else {
                    Ok(lineguard::fixer::FixResult {
                        file_path: file_path.clone(),
                        fixed: false,
                        issues_fixed: vec![],
                    })
                };

                if let Some(pb) = &pb_mutex {
                    if let Ok(pb) = pb.lock() {
                        pb.inc(1);
                    }
                }

                (check_result, fix_result)
            })
            .collect();

        if let Some(pb) = progress_bar {
            pb.finish_and_clear();
        }

        // Report fix results
        report_fix_results(&fix_results, &args);

        // Exit with appropriate code
        let has_errors = fix_results
            .iter()
            .any(|(_, fix_result)| fix_result.is_err());
        process::exit(if has_errors { 1 } else { 0 });
    } else {
        // Normal check mode
        let all_results: Vec<_> = files
            .par_iter()
            .map(|file_path| {
                let result = check_file(file_path, &config_arc);
                if let Some(pb) = &pb_mutex {
                    if let Ok(pb) = pb.lock() {
                        pb.inc(1);
                    }
                }
                result
            })
            .collect();

        if let Some(pb) = progress_bar {
            pb.finish_and_clear();
        }

        // Create appropriate reporter
        let reporter: Box<dyn Reporter> = match args.format {
            OutputFormat::Json => Box::new(JsonReporter),
            OutputFormat::GitHub => Box::new(GitHubReporter),
            OutputFormat::Human => Box::new(HumanReporter {
                use_color: !args.no_color,
            }),
        };

        // Check for permission errors
        let permission_errors: Vec<_> = all_results.iter().filter(|r| r.error.is_some()).collect();

        // Report permission errors to stderr
        if !permission_errors.is_empty() && !args.quiet {
            for result in &permission_errors {
                if let Some(error) = &result.error {
                    eprintln!("{error}");
                }
            }
        }

        // Exit with appropriate code
        let has_issues = all_results.iter().any(|r| !r.issues.is_empty());

        // Report results (skip for quiet mode if no issues)
        if !args.quiet || has_issues {
            reporter.report(&all_results);
        }

        // Exit with 1 only if there are actual lint issues, not permission errors
        process::exit(if has_issues { 1 } else { 0 });
    }
}

fn report_fix_results(
    results: &[(
        lineguard::CheckResult,
        Result<lineguard::fixer::FixResult, anyhow::Error>,
    )],
    args: &lineguard::cli::CliArgs,
) {
    if args.quiet {
        return;
    }

    let mut fixed_count = 0;
    let mut error_count = 0;

    for (check_result, fix_result) in results {
        match fix_result {
            Ok(fix) if fix.fixed => {
                fixed_count += 1;
                if args.format == OutputFormat::Human {
                    if args.dry_run {
                        println!("Would fix: {}", fix.file_path.display());
                    } else {
                        println!("Fixed: {}", fix.file_path.display());
                    }
                }
            },
            Err(e) => {
                error_count += 1;
                if args.format == OutputFormat::Human {
                    eprintln!("{}: {}", check_result.file_path.display(), e);
                }
            },
            _ => {},
        }
    }

    if args.format == OutputFormat::Human && fixed_count > 0 {
        if args.dry_run {
            println!(
                "\nWould fix {} file{}",
                fixed_count,
                if fixed_count == 1 { "" } else { "s" }
            );
        } else {
            println!(
                "\nFixed {} file{}",
                fixed_count,
                if fixed_count == 1 { "" } else { "s" }
            );
        }
    }

    if error_count > 0 {
        eprintln!(
            "\n{} error{} occurred",
            error_count,
            if error_count == 1 { "" } else { "s" }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineguard::checker::{CheckResult, Issue, IssueType};
    use lineguard::cli::{CliArgs, OutputFormat};
    use lineguard::fixer::FixResult;
    use std::path::PathBuf;

    fn create_test_args(format: OutputFormat, quiet: bool, dry_run: bool) -> CliArgs {
        CliArgs {
            files: vec![],
            stdin: false,
            recursive: false,
            ignore: vec![],
            extensions: None,
            format,
            config: None,
            fix: false,
            dry_run,
            quiet,
            no_color: true,
            verbose: false,
            no_newline_check: false,
            no_trailing_space: false,
            from: None,
            to: None,
            no_hidden: false,
        }
    }

    fn create_check_result(file_path: PathBuf, has_issues: bool) -> CheckResult {
        CheckResult {
            file_path: file_path.clone(),
            issues: if has_issues {
                vec![Issue {
                    line: Some(1),
                    issue_type: IssueType::MissingNewline,
                    message: "Missing newline at end of file".to_string(),
                }]
            } else {
                vec![]
            },
            error: None,
        }
    }

    fn create_fix_result(file_path: PathBuf, fixed: bool) -> FixResult {
        FixResult {
            file_path,
            fixed,
            issues_fixed: if fixed {
                vec![Issue {
                    line: Some(1),
                    issue_type: IssueType::MissingNewline,
                    message: "Missing newline at end of file".to_string(),
                }]
            } else {
                vec![]
            },
        }
    }

    #[test]
    fn test_report_fix_results_quiet_mode() {
        // Test that no output is produced in quiet mode
        let file_path = PathBuf::from("test.txt");
        let check_result = create_check_result(file_path.clone(), true);
        let fix_result = Ok(create_fix_result(file_path, true));
        let results = vec![(check_result, fix_result)];
        let args = create_test_args(OutputFormat::Human, true, false);

        // In quiet mode, the function should return without producing output
        report_fix_results(&results, &args);
    }

    #[test]
    fn test_report_fix_results_human_format_fixed() {
        let file_path = PathBuf::from("test.txt");
        let check_result = create_check_result(file_path.clone(), true);
        let fix_result = Ok(create_fix_result(file_path, true));
        let results = vec![(check_result, fix_result)];
        let args = create_test_args(OutputFormat::Human, false, false);

        // This should produce output - "Fixed: test.txt" and "\nFixed 1 file"
        report_fix_results(&results, &args);
    }

    #[test]
    fn test_report_fix_results_dry_run() {
        let file_path = PathBuf::from("test.txt");
        let check_result = create_check_result(file_path.clone(), true);
        let fix_result = Ok(create_fix_result(file_path, true));
        let results = vec![(check_result, fix_result)];
        let args = create_test_args(OutputFormat::Human, false, true);

        // This should produce output - "Would fix: test.txt" and "\nWould fix 1 file"
        report_fix_results(&results, &args);
    }

    #[test]
    fn test_report_fix_results_with_errors() {
        let file_path = PathBuf::from("test.txt");
        let check_result = create_check_result(file_path.clone(), true);
        let fix_result = Err(anyhow::anyhow!("Permission denied"));
        let results = vec![(check_result, fix_result)];
        let args = create_test_args(OutputFormat::Human, false, false);

        // This should produce error output
        report_fix_results(&results, &args);
    }

    #[test]
    fn test_report_fix_results_json_format() {
        let file_path = PathBuf::from("test.txt");
        let check_result = create_check_result(file_path.clone(), true);
        let fix_result = Ok(create_fix_result(file_path, true));
        let results = vec![(check_result, fix_result)];
        let args = create_test_args(OutputFormat::Json, false, false);

        // JSON format should not produce human-readable output
        report_fix_results(&results, &args);
    }

    #[test]
    fn test_report_fix_results_multiple_files() {
        let file1 = PathBuf::from("test1.txt");
        let file2 = PathBuf::from("test2.txt");
        let check_result1 = create_check_result(file1.clone(), true);
        let check_result2 = create_check_result(file2.clone(), true);
        let fix_result1 = Ok(create_fix_result(file1, true));
        let fix_result2 = Ok(create_fix_result(file2, true));
        let results = vec![(check_result1, fix_result1), (check_result2, fix_result2)];
        let args = create_test_args(OutputFormat::Human, false, false);

        // Should show "Fixed 2 files"
        report_fix_results(&results, &args);
    }
}

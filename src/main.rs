use indicatif::{ProgressBar, ProgressStyle};
use lineguard::checker::check_file;
use lineguard::cli::{OutputFormat, parse_args};
use lineguard::config::load_config;
use lineguard::discovery::discover_files;
use lineguard::reporter::{GitHubReporter, HumanReporter, JsonReporter, Reporter};
use rayon::prelude::*;
use std::process;
use std::sync::Mutex;

fn main() {
    let args = parse_args();

    // Load configuration
    let config = match load_config(args.config.as_deref()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {e}");
            process::exit(4);
        },
    };

    // Discover files to check
    let files = match discover_files(&args, &config) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(3);
        },
    };

    if files.is_empty() && !args.quiet {
        eprintln!("No files found to check");
        process::exit(0);
    }

    // Show checking message for human format
    if !args.quiet && args.format == OutputFormat::Human && files.len() > 1 {
        println!("Checking {} files...", files.len());
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

    // Check each file in parallel
    let all_results: Vec<_> = files
        .par_iter()
        .map(|file_path| {
            let result = check_file(file_path, &config);
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

    // Exit with appropriate code
    let has_issues = all_results.iter().any(|r| !r.issues.is_empty());

    // Report results (skip for quiet mode if no issues)
    if !args.quiet || has_issues {
        reporter.report(&all_results);
    }

    process::exit(if has_issues { 1 } else { 0 });
}

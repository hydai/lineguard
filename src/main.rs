use lineguard::checker::check_file;
use lineguard::cli::{OutputFormat, parse_args};
use lineguard::discovery::discover_files;
use lineguard::reporter::{GitHubReporter, HumanReporter, JsonReporter, Reporter};
use rayon::prelude::*;
use std::process;

fn main() {
    let args = parse_args();

    // Discover files to check
    let files = match discover_files(&args) {
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

    // Check each file in parallel
    let all_results: Vec<_> = files
        .par_iter()
        .map(|file_path| check_file(file_path))
        .collect();

    // Create appropriate reporter
    let reporter: Box<dyn Reporter> = match args.format {
        OutputFormat::Json => Box::new(JsonReporter),
        OutputFormat::GitHub => Box::new(GitHubReporter),
        OutputFormat::Human => Box::new(HumanReporter {
            use_color: !args.no_color,
        }),
    };

    // Report results
    reporter.report(&all_results);

    // Exit with appropriate code
    let has_issues = all_results.iter().any(|r| !r.issues.is_empty());
    process::exit(if has_issues { 1 } else { 0 });
}

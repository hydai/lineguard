use crate::CheckResult;

pub trait Reporter {
    fn report(&self, results: &[CheckResult]);
}

pub struct HumanReporter {
    pub use_color: bool,
}

pub struct JsonReporter;

pub struct GitHubReporter;

impl Reporter for HumanReporter {
    fn report(&self, results: &[CheckResult]) {
        let mut total_issues = 0;
        let mut files_with_issues = 0;

        for result in results {
            if !result.issues.is_empty() {
                files_with_issues += 1;
                total_issues += result.issues.len();

                if self.use_color {
                    use colored::*;
                    println!("{} {}", "✗".red(), result.file_path.display());
                } else {
                    println!("✗ {}", result.file_path.display());
                }

                for issue in &result.issues {
                    match issue.line {
                        Some(line) => println!("  - Line {}: {}", line, issue.message),
                        None => println!("  - {}", issue.message),
                    }
                }
                println!();
            }
        }

        // Summary
        if total_issues == 0 {
            if self.use_color {
                use colored::*;
                println!("{} All files passed lint checks!", "✓".green());
            } else {
                println!("✓ All files passed lint checks!");
            }
            println!("  Files checked: {}", results.len());
        } else {
            if self.use_color {
                use colored::*;
                println!(
                    "{} Found {} issues in {} files",
                    "✗".red(),
                    total_issues,
                    files_with_issues
                );
            } else {
                println!("✗ Found {total_issues} issues in {files_with_issues} files");
            }
            println!("  Files checked: {}", results.len());
        }
    }
}

impl Reporter for JsonReporter {
    fn report(&self, results: &[CheckResult]) {
        use serde_json::json;

        let files_checked = results.len();
        let files_with_issues = results.iter().filter(|r| !r.issues.is_empty()).count();
        let total_issues: usize = results.iter().map(|r| r.issues.len()).sum();

        let mut issues = Vec::new();
        for result in results {
            if !result.issues.is_empty() {
                let file_issues: Vec<_> = result
                    .issues
                    .iter()
                    .map(|issue| {
                        json!({
                            "type": match issue.issue_type {
                                crate::IssueType::MissingNewline => "missing_newline",
                                crate::IssueType::MultipleNewlines => "multiple_newlines",
                                crate::IssueType::TrailingSpace => "trailing_space",
                            },
                            "line": issue.line,
                            "message": issue.message,
                        })
                    })
                    .collect();

                issues.push(json!({
                    "file": result.file_path.display().to_string(),
                    "issues": file_issues,
                }));
            }
        }

        let output = json!({
            "files_checked": files_checked,
            "files_with_issues": files_with_issues,
            "total_issues": total_issues,
            "issues": issues,
        });

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}

impl Reporter for GitHubReporter {
    fn report(&self, results: &[CheckResult]) {
        for result in results {
            for issue in &result.issues {
                let file = result.file_path.display();
                match issue.line {
                    Some(line) => {
                        println!("::error file={},line={}::{}", file, line, issue.message);
                    },
                    None => {
                        println!("::error file={}::{}", file, issue.message);
                    },
                }
            }
        }
    }
}

use crate::config::Config;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub file_path: std::path::PathBuf,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Issue {
    pub issue_type: IssueType,
    pub line: Option<usize>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IssueType {
    MissingNewline,
    MultipleNewlines,
    TrailingSpace,
}

pub fn check_file(path: &Path, config: &Config) -> CheckResult {
    // For small files (< 10MB), use the existing implementation
    let file_size = match fs::metadata(path) {
        Ok(metadata) => metadata.len(),
        Err(_) => {
            return CheckResult {
                file_path: path.to_path_buf(),
                issues: vec![],
            };
        },
    };

    // Use streaming for files larger than 10MB
    if file_size > 10 * 1024 * 1024 {
        check_file_streaming(path, config)
    } else {
        check_file_in_memory(path, config)
    }
}

fn check_file_in_memory(path: &Path, config: &Config) -> CheckResult {
    let mut issues = Vec::new();

    // Read file content
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            // If we can't read the file, return empty result
            return CheckResult {
                file_path: path.to_path_buf(),
                issues,
            };
        },
    };

    // Check newline ending if enabled
    if config.checks.newline_ending {
        if let Some(issue) = check_newline_ending(&content) {
            issues.push(issue);
        }
    }

    // Check trailing spaces if enabled
    if config.checks.trailing_spaces {
        let mut trailing_space_issues = check_trailing_spaces(&content);
        issues.append(&mut trailing_space_issues);
    }

    CheckResult {
        file_path: path.to_path_buf(),
        issues,
    }
}

fn check_file_streaming(path: &Path, config: &Config) -> CheckResult {
    let mut issues = Vec::new();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return CheckResult {
                file_path: path.to_path_buf(),
                issues,
            };
        },
    };

    let reader = BufReader::new(file);
    let mut line_number = 0;
    let mut has_content = false;

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                has_content = true;
                line_number += 1;

                // Check trailing spaces if enabled
                if config.checks.trailing_spaces && line.trim_end().len() < line.len() {
                    issues.push(Issue {
                        issue_type: IssueType::TrailingSpace,
                        line: Some(line_number),
                        message: "Trailing spaces found".to_string(),
                    });
                }
            },
            Err(_) => break,
        }
    }

    // Check newline ending if enabled
    if config.checks.newline_ending && has_content {
        // For streaming, we need to check the actual file ending
        // Since BufRead strips newlines, we need to check the raw file
        if let Ok(mut file) = File::open(path) {
            // Read last few bytes to check for newline
            let _ = file.seek(SeekFrom::End(-2));
            let mut buffer = [0u8; 2];
            if let Ok(bytes_read) = file.read(&mut buffer) {
                let end_bytes = &buffer[..bytes_read];

                // Check if file ends with newline
                let ends_with_newline = end_bytes.last() == Some(&b'\n');
                let ends_with_double_newline = bytes_read == 2 && end_bytes == b"\n\n";

                if !ends_with_newline {
                    issues.push(Issue {
                        issue_type: IssueType::MissingNewline,
                        line: None,
                        message: "Missing newline at end of file".to_string(),
                    });
                } else if ends_with_double_newline {
                    issues.push(Issue {
                        issue_type: IssueType::MultipleNewlines,
                        line: None,
                        message: "Multiple newlines at end of file".to_string(),
                    });
                }
            }
        }
    }

    CheckResult {
        file_path: path.to_path_buf(),
        issues,
    }
}

pub fn check_newline_ending(content: &str) -> Option<Issue> {
    if content.is_empty() {
        // Empty files are considered valid
        return None;
    }

    if !content.ends_with('\n') {
        Some(Issue {
            issue_type: IssueType::MissingNewline,
            line: None,
            message: "Missing newline at end of file".to_string(),
        })
    } else if content.ends_with("\n\n") {
        Some(Issue {
            issue_type: IssueType::MultipleNewlines,
            line: None,
            message: "Multiple newlines at end of file".to_string(),
        })
    } else {
        None
    }
}

pub fn check_trailing_spaces(content: &str) -> Vec<Issue> {
    let mut issues = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim_end();
        if trimmed.len() < line.len() {
            issues.push(Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(line_num + 1), // Line numbers are 1-based
                message: "Trailing spaces found".to_string(),
            });
        }
    }

    issues
}

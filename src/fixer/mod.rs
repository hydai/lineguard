use crate::config::Config;
use crate::{Issue, IssueType};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub struct FixResult {
    pub file_path: std::path::PathBuf,
    pub fixed: bool,
    pub issues_fixed: Vec<Issue>,
}

pub fn fix_file(
    path: &Path,
    issues: &[Issue],
    config: &Config,
    dry_run: bool,
) -> Result<FixResult, anyhow::Error> {
    // Check file size
    let file_size = fs::metadata(path)?.len();

    // Use streaming for files larger than 10MB
    if file_size > 10 * 1024 * 1024 {
        fix_file_streaming(path, issues, config, dry_run)
    } else {
        fix_file_in_memory(path, issues, config, dry_run)
    }
}

fn fix_file_in_memory(
    path: &Path,
    issues: &[Issue],
    config: &Config,
    dry_run: bool,
) -> Result<FixResult, anyhow::Error> {
    let content = fs::read_to_string(path)?;
    let fixed_content = apply_fixes(&content, issues, config);

    let fixed = content != fixed_content;

    if fixed && !dry_run {
        fs::write(path, &fixed_content)?;
    }

    Ok(FixResult {
        file_path: path.to_path_buf(),
        fixed,
        issues_fixed: if fixed { issues.to_vec() } else { vec![] },
    })
}

fn fix_file_streaming(
    path: &Path,
    issues: &[Issue],
    config: &Config,
    dry_run: bool,
) -> Result<FixResult, anyhow::Error> {
    if dry_run {
        // For dry run, we just report what would be fixed
        return Ok(FixResult {
            file_path: path.to_path_buf(),
            fixed: !issues.is_empty(),
            issues_fixed: issues.to_vec(),
        });
    }

    let has_trailing_spaces = config.checks.trailing_spaces
        && issues
            .iter()
            .any(|i| i.issue_type == IssueType::TrailingSpace);

    let has_newline_issues = config.checks.newline_ending
        && issues.iter().any(|i| {
            matches!(
                i.issue_type,
                IssueType::MissingNewline | IssueType::MultipleNewlines
            )
        });

    if !has_trailing_spaces && !has_newline_issues {
        return Ok(FixResult {
            file_path: path.to_path_buf(),
            fixed: false,
            issues_fixed: vec![],
        });
    }

    // Create temporary file
    let temp_path = path.with_extension("tmp");

    {
        let input_file = File::open(path)?;
        let output_file = File::create(&temp_path)?;
        let reader = BufReader::new(input_file);
        let mut writer = BufWriter::new(output_file);

        let mut lines: Vec<String> = Vec::new();

        // Process lines
        for line_result in reader.lines() {
            let line = line_result?;
            if has_trailing_spaces {
                lines.push(line.trim_end().to_string());
            } else {
                lines.push(line);
            }
        }

        // Fix newline ending if needed
        if has_newline_issues && !lines.is_empty() {
            // Remove empty lines at the end
            while lines.len() > 1 && lines.last().is_some_and(|l| l.is_empty()) {
                lines.pop();
            }
        }

        // Write all lines
        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                writeln!(writer)?;
            }
            write!(writer, "{line}")?;
        }

        // Ensure file ends with newline
        if has_newline_issues || !lines.is_empty() {
            writeln!(writer)?;
        }

        writer.flush()?;
    }

    // Replace original file
    fs::rename(&temp_path, path)?;

    Ok(FixResult {
        file_path: path.to_path_buf(),
        fixed: true,
        issues_fixed: issues.to_vec(),
    })
}

fn apply_fixes(content: &str, issues: &[Issue], config: &Config) -> String {
    let mut result = content.to_string();

    // Fix trailing spaces if enabled
    if config.checks.trailing_spaces
        && issues
            .iter()
            .any(|i| i.issue_type == IssueType::TrailingSpace)
    {
        result = fix_trailing_spaces(&result);
    }

    // Fix newline issues if enabled
    if config.checks.newline_ending {
        let has_missing_newline = issues
            .iter()
            .any(|i| i.issue_type == IssueType::MissingNewline);
        let has_multiple_newlines = issues
            .iter()
            .any(|i| i.issue_type == IssueType::MultipleNewlines);

        if has_missing_newline || has_multiple_newlines {
            result = fix_newline_ending(&result);
        }
    }

    result
}

fn fix_trailing_spaces(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();

    for (i, line) in lines.iter().enumerate() {
        result.push_str(line.trim_end());
        if i < lines.len() - 1 {
            result.push('\n');
        }
    }

    // Preserve final newline if original had one
    if content.ends_with('\n') {
        result.push('\n');
    }

    result
}

fn fix_newline_ending(content: &str) -> String {
    let mut result = content.trim_end().to_string();
    result.push('\n');
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_trailing_spaces_only() {
        let content = "line 1  \nline 2   \nline 3\n";
        let expected = "line 1\nline 2\nline 3\n";
        assert_eq!(fix_trailing_spaces(content), expected);
    }

    #[test]
    fn test_fix_trailing_spaces_no_final_newline() {
        let content = "line 1  \nline 2   ";
        let expected = "line 1\nline 2";
        assert_eq!(fix_trailing_spaces(content), expected);
    }

    #[test]
    fn test_fix_newline_missing() {
        let content = "line 1\nline 2";
        let expected = "line 1\nline 2\n";
        assert_eq!(fix_newline_ending(content), expected);
    }

    #[test]
    fn test_fix_newline_multiple() {
        let content = "line 1\nline 2\n\n\n";
        let expected = "line 1\nline 2\n";
        assert_eq!(fix_newline_ending(content), expected);
    }

    #[test]
    fn test_fix_newline_already_correct() {
        let content = "line 1\nline 2\n";
        let expected = "line 1\nline 2\n";
        assert_eq!(fix_newline_ending(content), expected);
    }
}

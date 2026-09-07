use crate::config::Config;
use crate::{Issue, IssueType};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
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
        let mut reader = BufReader::new(File::open(path)?);
        let mut writer = BufWriter::new(File::create(&temp_path)?);

        // Line ending of the first line, used when one has to be added
        let mut file_line_ending: Option<String> = None;
        // Whether the last line written ended with a line ending
        let mut last_line_had_ending = false;
        // Every line is written as it is read. A run of blank lines is only
        // remembered by the byte offset where it started, so a run at the end
        // of the file can be cut off with set_len instead of being buffered.
        let mut bytes_written: u64 = 0;
        let mut blank_run_start: Option<u64> = None;
        let mut buf = Vec::new();

        loop {
            buf.clear();
            if reader.read_until(b'\n', &mut buf)? == 0 {
                break;
            }
            let line = std::str::from_utf8(&buf)?;
            let (body, ending) = split_line_ending(line);
            if file_line_ending.is_none() && !ending.is_empty() {
                file_line_ending = Some(ending.to_string());
            }

            let body = if has_trailing_spaces {
                body.trim_end()
            } else {
                body
            };

            if has_newline_issues && body.is_empty() {
                if blank_run_start.is_none() {
                    blank_run_start = Some(bytes_written);
                }
            } else {
                blank_run_start = None;
            }

            writer.write_all(body.as_bytes())?;
            writer.write_all(ending.as_bytes())?;
            bytes_written += (body.len() + ending.len()) as u64;
            last_line_had_ending = !ending.is_empty();
        }

        writer.flush()?;
        let mut file = writer.into_inner().map_err(|e| e.into_error())?;

        if has_newline_issues {
            let ending = file_line_ending.as_deref().unwrap_or("\n");
            if let Some(start) = blank_run_start {
                // Blank lines at the end are cut off; a file of only blank
                // lines keeps a single line ending
                file.set_len(start)?;
                if start == 0 {
                    file.seek(SeekFrom::Start(0))?;
                    file.write_all(ending.as_bytes())?;
                }
            } else if bytes_written > 0 && !last_line_had_ending {
                // The last line must end with exactly one line ending
                file.write_all(ending.as_bytes())?;
            }
        }
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

/// Split a line into its content and its line ending (`\r\n`, `\n` or empty).
fn split_line_ending(line: &str) -> (&str, &str) {
    if let Some(body) = line.strip_suffix("\r\n") {
        (body, "\r\n")
    } else if let Some(body) = line.strip_suffix('\n') {
        (body, "\n")
    } else {
        (line, "")
    }
}

/// Line ending used by the first line of `content`, defaulting to `\n`.
fn detect_line_ending(content: &str) -> &'static str {
    match content.split_inclusive('\n').next() {
        Some(first) if first.ends_with("\r\n") => "\r\n",
        _ => "\n",
    }
}

fn fix_trailing_spaces(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    for line in content.split_inclusive('\n') {
        let (body, ending) = split_line_ending(line);
        result.push_str(body.trim_end());
        result.push_str(ending);
    }
    result
}

/// Remove one trailing line ending from `s`, returning the rest and the ending.
fn strip_one_line_ending(s: &str) -> Option<(&str, &'static str)> {
    if let Some(rest) = s.strip_suffix("\r\n") {
        Some((rest, "\r\n"))
    } else {
        s.strip_suffix('\n').map(|rest| (rest, "\n"))
    }
}

fn fix_newline_ending(content: &str) -> String {
    // Peel trailing line endings off one at a time and keep the innermost,
    // so the retained last line keeps the ending it already had
    let mut body = content;
    let mut ending = None;
    while let Some((rest, stripped)) = strip_one_line_ending(body) {
        body = rest;
        ending = Some(stripped);
    }

    let ending = ending.unwrap_or_else(|| detect_line_ending(content));
    format!("{body}{ending}")
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

    #[test]
    fn test_fix_trailing_spaces_preserves_crlf() {
        assert_eq!(
            fix_trailing_spaces("a  \r\nb\t\r\nc\r\n"),
            "a\r\nb\r\nc\r\n"
        );
    }

    #[test]
    fn test_fix_newline_missing_uses_crlf_when_file_is_crlf() {
        assert_eq!(fix_newline_ending("a\r\nb"), "a\r\nb\r\n");
    }

    #[test]
    fn test_fix_newline_multiple_crlf() {
        assert_eq!(fix_newline_ending("a\r\nb\r\n\r\n"), "a\r\nb\r\n");
    }

    #[test]
    fn test_fix_newline_ending_keeps_trailing_spaces() {
        // Trailing whitespace is the trailing-space fixer's job, not this one's
        assert_eq!(fix_newline_ending("a  "), "a  \n");
    }

    fn streaming_fix(content: &str, issues: &[Issue]) -> String {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("file.txt");
        fs::write(&path, content).unwrap();

        let result = fix_file_streaming(&path, issues, &Config::default(), false).unwrap();
        assert!(result.fixed);

        fs::read_to_string(&path).unwrap()
    }

    fn issue(issue_type: IssueType, line: Option<usize>) -> Issue {
        Issue {
            issue_type,
            line,
            message: String::new(),
        }
    }

    #[test]
    fn test_fix_file_streaming_preserves_crlf() {
        let fixed = streaming_fix(
            "a  \r\nb\r\n\r\n",
            &[
                issue(IssueType::TrailingSpace, Some(1)),
                issue(IssueType::MultipleNewlines, None),
            ],
        );
        assert_eq!(fixed, "a\r\nb\r\n");
    }

    #[test]
    fn test_fix_file_streaming_missing_newline_crlf() {
        let fixed = streaming_fix("a\r\nb", &[issue(IssueType::MissingNewline, None)]);
        assert_eq!(fixed, "a\r\nb\r\n");
    }

    #[test]
    fn test_fix_newline_multiple_keeps_the_last_line_ending_in_mixed_file() {
        assert_eq!(fix_newline_ending("a\r\nb\n\n"), "a\r\nb\n");
    }

    #[test]
    fn test_fix_file_streaming_keeps_the_last_line_ending_in_mixed_file() {
        let fixed = streaming_fix("a\r\nb\n\n", &[issue(IssueType::MultipleNewlines, None)]);
        assert_eq!(fixed, "a\r\nb\n");
    }

    #[test]
    fn test_fix_file_streaming_drops_only_the_trailing_blank_run() {
        let fixed = streaming_fix(
            "a\n\n\nb\n\n\n\n",
            &[issue(IssueType::MultipleNewlines, None)],
        );
        assert_eq!(fixed, "a\n\n\nb\n");
    }

    #[test]
    fn test_fix_file_streaming_blank_only_file_keeps_one_line_ending() {
        let fixed = streaming_fix("\r\n\r\n\r\n", &[issue(IssueType::MultipleNewlines, None)]);
        assert_eq!(fixed, "\r\n");
    }
}

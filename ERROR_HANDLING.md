# LineLint Error Handling and Reporting

## Error Categories

### 1. User Errors
These are errors caused by incorrect usage or invalid input.

#### Invalid Arguments
```
Error: Invalid argument combination
  Cannot use --stdin with file paths

Usage: linelint [OPTIONS] [FILES...]
Try 'linelint --help' for more information.
```

#### File Not Found
```
Error: File not found
  Cannot access 'src/missing.rs': No such file or directory
```

#### Invalid Pattern
```
Error: Invalid glob pattern
  Pattern '**[*.rs' is malformed: Unclosed character class
```

### 2. System Errors
These are errors from the operating system or file system.

#### Permission Denied
```
Error: Permission denied
  Cannot read file '/etc/shadow': Permission denied
```

#### IO Errors
```
Error: IO operation failed
  Failed to read 'large_file.txt': Interrupted system call
```

### 3. Configuration Errors
Errors related to configuration files or settings.

#### Invalid Config File
```
Error: Configuration error
  Failed to parse '.linelintrc': Expected boolean at line 5, column 12
```

#### Conflicting Settings
```
Error: Configuration conflict
  Cannot disable all checks. At least one check must be enabled.
```

## Error Reporting Format

### Standard Error Structure
```rust
pub struct ErrorReport {
    pub error_type: ErrorType,
    pub message: String,
    pub detail: Option<String>,
    pub hint: Option<String>,
    pub location: Option<ErrorLocation>,
}

pub struct ErrorLocation {
    pub file: Option<PathBuf>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}
```

### Human-Readable Error Format
```
[ERROR] {error_type}: {message}
  ├─ {detail}
  └─ Hint: {hint}
```

Example:
```
[ERROR] Pattern Error: Invalid glob pattern
  ├─ Pattern '**[*.rs' has unclosed character class
  └─ Hint: Did you mean '**/*.rs'?
```

### JSON Error Format
```json
{
  "type": "error",
  "error": {
    "type": "pattern_error",
    "message": "Invalid glob pattern",
    "detail": "Pattern '**[*.rs' has unclosed character class",
    "hint": "Did you mean '**/*.rs'?",
    "location": null
  }
}
```

### GitHub Actions Error Format
```
::error::Invalid glob pattern: Pattern '**[*.rs' has unclosed character class
```

## Issue Reporting Format

### Issue Types
1. **Missing Newline** - File doesn't end with newline
2. **Multiple Newlines** - File ends with multiple newlines
3. **Trailing Spaces** - Line ends with whitespace

### Human-Readable Issue Format

#### Compact Mode (Default)
```
src/main.rs
  ⚠ Missing newline at end of file
  ⚠ Line 45: Trailing spaces (3 spaces)
  ⚠ Line 67: Trailing spaces (1 tab)

tests/utils.rs
  ⚠ Line 23: Trailing spaces (2 spaces)
```

#### Verbose Mode
```
src/main.rs
  Issue: Missing newline at end of file
    Type: missing_newline
    Severity: warning
    
  Issue: Trailing spaces found
    Type: trailing_space
    Line: 45
    Detail: Found 3 trailing space characters
    Content: "    let result = process();   "
                                      ^^^
```

### JSON Issue Format
```json
{
  "file": "src/main.rs",
  "issues": [
    {
      "type": "missing_newline",
      "severity": "warning",
      "message": "Missing newline at end of file",
      "line": null,
      "column": null
    },
    {
      "type": "trailing_space",
      "severity": "warning",
      "message": "Trailing spaces found",
      "line": 45,
      "column": 28,
      "detail": "3 space characters"
    }
  ]
}
```

### Summary Report

#### Success Case
```
✓ All files passed lint checks!
  Files checked: 156
  Time: 0.234s
```

#### Issues Found
```
✗ Found 12 issues in 5 files
  Files checked: 156
  Files with issues: 5
  Total issues: 12
    - Missing newlines: 2
    - Trailing spaces: 10
  Time: 0.234s
```

## Exit Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 0 | Success | All checks passed |
| 1 | Issues Found | Lint issues detected |
| 2 | Argument Error | Invalid command-line arguments |
| 3 | IO Error | File access or read error |
| 4 | Config Error | Configuration file error |
| 5 | Pattern Error | Invalid glob pattern |
| 126 | Permission Error | Cannot execute due to permissions |
| 127 | Not Found | Command or file not found |

## Error Recovery Strategies

### Partial Success
When checking multiple files, continue checking remaining files even if some fail:
```
Checking 100 files...
  ✓ 95 files checked successfully
  ✗ 3 files had issues
  ! 2 files skipped (errors)
    - config/secure.conf: Permission denied
    - data/large.bin: File too large
```

### Graceful Degradation
- If config file is invalid, use defaults and warn user
- If color output fails, fallback to plain text
- If progress bar fails, continue without it

### User-Friendly Hints
Provide actionable hints for common errors:
- Suggest correct glob patterns
- Recommend checking file permissions
- Propose alternative commands
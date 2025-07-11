# LineLint CLI Design

## Command Structure

### Basic Usage
```bash
linelint [OPTIONS] [FILES...]
```

### Examples

#### Check single file
```bash
linelint main.rs
```

#### Check multiple files
```bash
linelint src/main.rs src/lib.rs
```

#### Check with glob pattern
```bash
linelint "src/**/*.rs"
```

#### Check all files in directory
```bash
linelint .
```

#### Pipe files from find command
```bash
find . -name "*.rs" | linelint --stdin
```

## Command-Line Options

### Input Options
- `[FILES...]` - File paths or glob patterns to check
- `--stdin` - Read file paths from standard input
- `-r, --recursive` - Recursively check directories

### Check Options
- `--no-newline-check` - Disable newline ending check
- `--no-trailing-space` - Disable trailing space check
- `--fix` - Automatically fix issues (future enhancement)

### Output Options
- `-f, --format <FORMAT>` - Output format [default: human]
  - `human` - Human-readable output with colors
  - `json` - JSON format for programmatic use
  - `github` - GitHub Actions annotation format
- `-q, --quiet` - Suppress non-error output
- `-v, --verbose` - Show detailed information
- `--no-color` - Disable colored output

### Configuration
- `-c, --config <FILE>` - Path to configuration file
- `--ignore <PATTERN>` - Ignore files matching pattern
- `--extensions <EXT>` - File extensions to check (comma-separated)

### General Options
- `-h, --help` - Show help information
- `-V, --version` - Show version information

## Output Examples

### Human-Readable Format (Default)
```
Checking 3 files...

✗ src/main.rs
  - Missing newline at end of file
  - Line 45: Trailing spaces found

✗ tests/test_utils.rs
  - Line 23: Trailing spaces found
  - Line 67: Trailing spaces found

✓ src/lib.rs

Summary: 2 files with issues, 3 total issues found
```

### JSON Format
```json
{
  "files_checked": 3,
  "files_with_issues": 2,
  "total_issues": 3,
  "issues": [
    {
      "file": "src/main.rs",
      "issues": [
        {
          "type": "missing_newline",
          "line": null,
          "message": "Missing newline at end of file"
        },
        {
          "type": "trailing_space",
          "line": 45,
          "message": "Trailing spaces found"
        }
      ]
    },
    {
      "file": "tests/test_utils.rs",
      "issues": [
        {
          "type": "trailing_space",
          "line": 23,
          "message": "Trailing spaces found"
        },
        {
          "type": "trailing_space",
          "line": 67,
          "message": "Trailing spaces found"
        }
      ]
    }
  ]
}
```

### GitHub Actions Format
```
::error file=src/main.rs::Missing newline at end of file
::error file=src/main.rs,line=45::Trailing spaces found
::error file=tests/test_utils.rs,line=23::Trailing spaces found
::error file=tests/test_utils.rs,line=67::Trailing spaces found
```

## Exit Codes
- `0` - Success, no issues found
- `1` - Issues found in checked files
- `2` - Error in command-line arguments
- `3` - File access or I/O error
- `4` - Configuration error

## Progress Indication
For large file sets, show progress:
```
Checking files... [████████████████────] 80% (8000/10000)
```

## Error Handling
Clear error messages for common issues:
```
Error: Cannot access file 'src/missing.rs': Permission denied
Error: Invalid glob pattern '**[*.rs'
Error: No files found matching pattern '*.xyz'
```
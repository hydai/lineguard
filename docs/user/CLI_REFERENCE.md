# LineGuard CLI Design

## Command Structure

### Basic Usage
```bash
lineguard [OPTIONS] [FILES...]
```

### Examples

#### Check single file
```bash
lineguard main.rs
```

#### Check multiple files
```bash
lineguard src/main.rs src/lib.rs
```

#### Check with glob pattern
```bash
lineguard "src/**/*.rs"
```

#### Check all files in directory
```bash
lineguard .
```

#### Pipe files from find command
```bash
find . -name "*.rs" | lineguard --stdin
```

#### Fix issues automatically
```bash
lineguard --fix src/
```

#### Preview fixes without applying
```bash
lineguard --fix --dry-run src/
```

#### Check files changed in the last commit
```bash
lineguard --from HEAD~1 .
```

#### Check files changed between commits
```bash
lineguard --from abc123 --to def456 .
```

#### Check files changed since a tag
```bash
lineguard --from v1.0.0 src/
```

#### Check git range with verbose output
```bash
lineguard --verbose --from HEAD~5 --to HEAD .
# Output includes:
# Git range: abc1234..def5678
# Changed files: 10
#   - src/main.rs
#   - src/lib.rs
#   - ...
```

## Command-Line Options

### Input Options
- `[FILES...]` - File paths or glob patterns to check
- `--stdin` - Read file paths from standard input
- `-r, --recursive` - Recursively check directories

### Check Options
- `--no-newline-check` - Disable newline ending check
- `--no-trailing-space` - Disable trailing space check
- `--fix` - Automatically fix issues
- `--dry-run` - Show what would be fixed without modifying files
- `--from <COMMIT>` - Check only files changed since this commit (Git repositories only)
- `--to <COMMIT>` - Check files changed until this commit (Git only, default: HEAD)

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
- `--no-hidden` - Skip hidden files (files starting with .)
- `--no-gitignore` - Do not respect `.gitignore` files when discovering files in directories

### General Options
- `-h, --help` - Show help information
- `-V, --version` - Show version information

## Output Examples

### Human-Readable Format (Default)
```
Checking 3 files...
✗ src/main.rs
  - Line 45: Trailing spaces found
  - Missing newline at end of file

✗ tests/cli_tests.rs
  - Line 23: Trailing spaces found
  - Line 67: Trailing spaces found

✗ Found 4 issues in 2 files
  Files checked: 3
```

### JSON Format
```json
{
  "files_checked": 3,
  "files_with_issues": 2,
  "total_issues": 4,
  "issues": [
    {
      "file": "src/main.rs",
      "issues": [
        {
          "type": "trailing_space",
          "line": 45,
          "message": "Trailing spaces found"
        },
        {
          "type": "missing_newline",
          "line": null,
          "message": "Missing newline at end of file"
        }
      ]
    },
    {
      "file": "tests/cli_tests.rs",
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
  ],
  "errors": [
    {
      "file": "src/protected.rs",
      "error": "Permission denied (os error 13)"
    }
  ]
}
```

### GitHub Actions Format
```
::error file=src/main.rs,line=45::Trailing spaces found
::error file=src/main.rs::Missing newline at end of file
::error file=tests/cli_tests.rs,line=23::Trailing spaces found
::error file=tests/cli_tests.rs,line=67::Trailing spaces found
```

## Exit Codes
- `0` - Success, no issues found
- `1` - Issues found in checked files
- `2` - Error in command-line arguments
- `3` - File access or I/O error
- `4` - Configuration error

## Progress Indication
With more than 10 files in the human format (and without `--quiet`), a progress bar is shown while checking:
```
Checking files... [████████████████████████████████░░░░░░░░] 8000/10000 (80%)
```

## Error Handling
Per-file problems go to stderr and the remaining files are still checked; configuration and git problems abort the run:
```
src/protected.rs: Permission denied (os error 13)
No files found to check
Error: not a git repository
Error loading configuration: Configuration file not found: missing.toml
```
See [ERROR_HANDLING.md](../developer/ERROR_HANDLING.md) for exit codes and how each output format carries errors.

# LineGuard

[![CI](https://github.com/hydai/lineguard/workflows/CI/badge.svg)](https://github.com/hydai/lineguard/actions/workflows/ci.yml)
[![Release](https://github.com/hydai/lineguard/workflows/Release/badge.svg)](https://github.com/hydai/lineguard/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/lineguard.svg)](https://crates.io/crates/lineguard)
[![License](https://img.shields.io/crates/l/lineguard.svg)](LICENSE)

A fast and reliable file linter that ensures proper line endings and clean formatting.

## Features

- ✅ **Newline Ending Check**: Ensures files end with exactly one newline character
- ✅ **Trailing Space Detection**: Identifies and reports trailing whitespace at line ends
- 🚀 **High Performance**: Parallel file processing with progress indicators
- 🎨 **Multiple Output Formats**: Human-readable (with colors), JSON, and GitHub Actions formats
- 🔧 **Configurable**: Flexible configuration via CLI flags or `.lineguardrc` files
- 🔄 **Auto-fix**: Automatically fix issues with `--fix` flag
- 📁 **Smart File Discovery**: Glob patterns, recursive directory scanning, stdin support
- 🎯 **Selective Checks**: Disable specific checks via CLI flags
- 💾 **Memory Efficient**: Streaming support for large files (>10MB)
- 🛡️ **Robust Error Handling**: Graceful handling of permission errors
- 🔍 **Binary File Detection**: Automatically skips binary files
- 👁️ **Hidden Files**: Checks hidden files by default (use `--no-hidden` to skip)
- 🚫 **Ignore Patterns**: Skip files/directories with glob patterns
- 📝 **File Extension Filtering**: Check only specific file types
- 🔀 **Git Integration**: Check only files changed between commits

## Installation

### From Crates.io

```bash
cargo install lineguard
```

### From GitHub Releases

Download pre-built binaries from the [latest release](https://github.com/hydai/lineguard/releases/latest):

```bash
# Linux/macOS
curl -L https://github.com/hydai/lineguard/releases/latest/download/lineguard-{VERSION}-{TARGET}.tar.gz | tar xz
sudo mv lineguard /usr/local/bin/

# Windows
# Download the .zip file and extract lineguard.exe to a directory in your PATH
```

### From Source

```bash
git clone https://github.com/hydai/lineguard
cd lineguard
cargo install --path .
```

### Shell Command Alternative

If you prefer not to install LineGuard, you can achieve similar functionality using standard Unix tools:

```bash
# Basic check for trailing spaces and missing newlines
find . -name "*.txt" -type f -exec bash -c 'f="$1"; grep -n "[[:space:]]$" "$f" 2>/dev/null && echo "$f: has trailing spaces"; [ -n "$(tail -c 1 "$f")" ] && echo "$f: missing newline"' _ {} \;

# Fix issues automatically
find . -name "*.txt" -type f -exec bash -c 'sed -i.bak "s/[[:space:]]*$//" "$1" && rm "$1.bak"; [ -n "$(tail -c 1 "$1")" ] && echo >> "$1"' _ {} \;

# Check files changed in git
git diff --name-only master HEAD | while read f; do [ -f "$f" ] && { grep -q "[[:space:]]$" "$f" && echo "$f: trailing spaces"; [ -n "$(tail -c 1 "$f")" ] && echo "$f: missing newline"; }; done
```

See [Shell Alternatives](#shell-alternatives) section for more examples.

## Usage

### Basic Usage

```bash
# Check a single file
lineguard main.rs

# Check multiple files
lineguard src/main.rs src/lib.rs

# Check with glob pattern
lineguard "src/**/*.rs"

# Check all files in directory
lineguard .
```

### Command-Line Options

```bash
lineguard [OPTIONS] [FILES...]

Arguments:
  [FILES...]  Files or directories to check

Options:
  -r, --recursive              Recursively check directories
  -f, --format <FORMAT>        Output format [default: human] [possible values: human, json, github]
  -q, --quiet                  Suppress non-error output
  -v, --verbose                Show detailed information
      --no-color               Disable colored output
  -c, --config <CONFIG>        Path to configuration file
      --stdin                  Read file paths from stdin
      --ignore <IGNORE>        Ignore files matching pattern (can be used multiple times)
      --extensions <EXTENSIONS> File extensions to check (comma-separated)
      --no-hidden              Skip hidden files (files starting with .)
      --no-newline-check       Disable newline ending check
      --no-trailing-space      Disable trailing space check
      --fix                    Automatically fix issues
      --dry-run                Show what would be fixed without modifying files
      --from <FROM>            Check files changed since this commit (Git only)
      --to <TO>                Check files changed until this commit (Git only, default: HEAD)
  -h, --help                   Print help
  -V, --version                Print version
```

### Advanced Usage

```bash
# Fix issues automatically
lineguard --fix src/

# Preview fixes without applying
lineguard --fix --dry-run src/

# Ignore specific patterns
lineguard --ignore "*.generated.rs" --ignore "**/target/**" .

# Check only specific file types
lineguard --extensions rs,toml .

# Use custom config file
lineguard --config path/to/.lineguardrc src/

# Disable specific checks
lineguard --no-trailing-space src/  # Only check newlines
lineguard --no-newline-check src/   # Only check trailing spaces

# Pipe files from other commands
find . -name "*.rs" | lineguard --stdin

# Check files changed in the last commit
lineguard --from HEAD~1 .

# Check files changed between specific commits
lineguard --from abc123 --to def456 .

# Check files changed since a tag
lineguard --from v1.0.0 src/

# Combine with other options
lineguard --from main --fix --format json .
```

### Output Examples

**Human-Readable Format (Default)**
```
✗ src/main.rs
  - Missing newline at end of file
  - Line 45: Trailing spaces found

✓ src/lib.rs

Summary: 1 file with issues, 2 total issues found
```

**JSON Format**
```bash
lineguard --format json src/
```

```json
{
  "files_checked": 3,
  "files_with_issues": 1,
  "total_issues": 2,
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

**GitHub Actions Format**
```bash
lineguard --format github src/
```

## Configuration File

LineGuard supports configuration files to customize its behavior. Create a `.lineguardrc` file in your project root (see `.lineguardrc.example` for reference):

```toml
# .lineguardrc
[checks]
newline_ending = true      # Check for proper newline at end of file
trailing_spaces = true     # Check for trailing spaces

# Ignore patterns (glob format)
ignore_patterns = [
    "**/target/**",
    "**/.git/**",
    "**/node_modules/**",
    "*.generated.*",
]

# File extensions to check (default: all text files)
file_extensions = [
    "rs", "toml", "md", "txt",
    "js", "ts", "jsx", "tsx",
    "py", "go", "java", "c", "cpp", "h", "hpp",
    "yml", "yaml", "json", "xml",
]
```

Configuration files are searched in the following order:
1. Path specified with `--config` flag
2. `.lineguardrc` in the current directory
3. `.lineguardrc` in parent directories (up to the root)

CLI flags always override configuration file settings.

## Exit Codes

- `0` - Success, no issues found
- `1` - Issues found in checked files
- `2` - Error in command-line arguments
- `3` - File access or I/O error
- `4` - Configuration error

## Development

This project follows strict Test-Driven Development (TDD) practices. See [IMPLEMENTATION_PLAN.md](docs/planning/IMPLEMENTATION_PLAN.md) for development guidelines.

### Building

```bash
cargo build
```

### Testing

LineGuard uses comprehensive testing with dependency injection patterns for better testability.

#### Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests (fast)
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run tests for a specific module
cargo test reporter::

# Run tests with output
cargo test -- --nocapture
```

#### Test Coverage

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report (HTML)
cargo tarpaulin --lib --out html

# Generate coverage report (stdout)
cargo tarpaulin --lib --print-summary

# Check coverage for all tests
cargo tarpaulin --all
```

Current coverage targets:
- Overall: 90%+
- Core modules (checker, reporter): 85%+
- Utility modules: 80%+

#### Test Architecture

The codebase uses dependency injection and mock implementations:

```rust
// Using MockFileSystem for testing file operations
use lineguard::testing::mocks::MockFileSystem;

let mut fs = MockFileSystem::new();
fs.add_file("test.txt", "content\n");
fs.add_metadata("test.txt", MockMetadata::file(7));

// Using MockOutput for testing output operations
use lineguard::testing::mocks::MockOutput;

let mut output = MockOutput::new();
reporter.report_to(&results, &mut output)?;
assert_eq!(output.buffer, vec!["expected output\n"]);
```

### Code Quality

```bash
cargo fmt --all -- --check  # Check code formatting
cargo fmt --all             # Auto-format code
cargo clippy --all-targets --all-features -- -D warnings  # Run linter
```

## Shell Alternatives

While LineGuard provides superior performance and features, you can achieve similar basic functionality using standard Unix tools. These examples use `find`, `grep`, `sed`, and other common utilities.

### Basic Checks

```bash
# Check single file for issues
check_file() {
    local file="$1"
    grep -n '[[:space:]]$' "$file" 2>/dev/null && echo "$file: has trailing spaces"
    [ -n "$(tail -c 1 "$file")" ] && echo "$file: missing newline at EOF"
}

# Check all text files in current directory
find . -type f -name "*.txt" -exec bash -c '
    f="$1"
    grep -n "[[:space:]]$" "$f" 2>/dev/null && echo "$f: has trailing spaces"
    [ -n "$(tail -c 1 "$f")" ] && echo "$f: missing newline"
' _ {} \;

# Recursive check with multiple extensions
find . -type f \( -name "*.rs" -o -name "*.md" -o -name "*.txt" \) -exec bash -c '
    echo -n "Checking $1... "
    issues=0
    grep -q "[[:space:]]$" "$1" && { echo -n "trailing spaces, "; issues=1; }
    [ -n "$(tail -c 1 "$1")" ] && { echo -n "missing newline, "; issues=1; }
    [ $issues -eq 0 ] && echo "OK" || echo "FAILED"
' _ {} \;
```

### Fix Issues

```bash
# Remove trailing spaces from files
find . -name "*.txt" -type f -exec sed -i 's/[[:space:]]*$//' {} \;

# Add missing newlines
find . -name "*.txt" -type f -exec bash -c '[ -n "$(tail -c 1 "$1")" ] && echo >> "$1"' _ {} \;

# Fix both issues at once
fix_file() {
    local file="$1"
    # Remove trailing spaces
    sed -i.bak 's/[[:space:]]*$//' "$file" && rm "$file.bak"
    # Add newline if missing
    [ -n "$(tail -c 1 "$file")" ] && echo >> "$file"
}

# Apply fixes to all matching files
find . -name "*.txt" -type f -exec bash -c '
    sed -i.bak "s/[[:space:]]*$//" "$1" && rm "$1.bak"
    [ -n "$(tail -c 1 "$1")" ] && echo >> "$1"
' _ {} \;
```

### Git Integration

```bash
# Check files changed in last commit
git diff --name-only HEAD~1 HEAD | while read f; do
    [ -f "$f" ] && {
        grep -q '[[:space:]]$' "$f" && echo "$f: trailing spaces"
        [ -n "$(tail -c 1 "$f")" ] && echo "$f: missing newline"
    }
done

# Check files changed between branches
git diff --name-only master feature-branch | while read f; do
    [ -f "$f" ] && check_file "$f"
done

# Check staged files before commit
git diff --cached --name-only | while read f; do
    [ -f "$f" ] && {
        grep -n '[[:space:]]$' "$f" && { echo "$f has trailing spaces"; exit 1; }
        [ -n "$(tail -c 1 "$f")" ] && { echo "$f missing newline"; exit 1; }
    }
done
```

### Advanced Examples

```bash
# Parallel processing with GNU parallel (if installed)
find . -name "*.txt" -type f | parallel -j+0 '
    echo -n "{}: "
    grep -c "[[:space:]]$" {} | { read n; [ $n -gt 0 ] && echo -n "$n trailing spaces, "; }
    [ -n "$(tail -c 1 {})" ] && echo -n "missing newline, "
    echo "checked"
' | grep -v "0 trailing.*checked$"

# Generate report similar to LineGuard
check_all() {
    local total=0 issues=0
    while IFS= read -r file; do
        ((total++))
        local has_issue=0
        if grep -q '[[:space:]]$' "$file"; then
            echo "✗ $file: trailing spaces on lines $(grep -n '[[:space:]]$' "$file" | cut -d: -f1 | tr '\n' ' ')"
            has_issue=1
        fi
        if [ -n "$(tail -c 1 "$file")" ]; then
            echo "✗ $file: missing newline at EOF"
            has_issue=1
        fi
        [ $has_issue -eq 1 ] && ((issues++))
        [ $has_issue -eq 0 ] && echo "✓ $file"
    done < <(find . -name "*.txt" -type f)
    echo
    echo "Checked $total files, found issues in $issues files"
    [ $issues -gt 0 ] && return 1 || return 0
}
```

### Shell Function for .bashrc/.zshrc

Add this function to your shell configuration for easy access:

```bash
lineguard_check() {
    local pattern="${1:-*}"
    local recursive="${2:-}"
    local find_depth=""

    [ "$recursive" != "-r" ] && find_depth="-maxdepth 1"

    find . $find_depth -name "$pattern" -type f ! -path '*/\.*' -exec bash -c '
        file="$1"
        has_issue=0

        # Check trailing spaces
        if trailing=$(grep -n "[[:space:]]$" "$file" 2>/dev/null); then
            echo -e "\033[0;31m✗\033[0m $file"
            echo "  Trailing spaces on lines:"
            echo "$trailing" | cut -d: -f1 | while read ln; do echo "    Line $ln"; done
            has_issue=1
        fi

        # Check newline at EOF
        if [ -s "$file" ] && [ -n "$(tail -c 1 "$file")" ]; then
            [ $has_issue -eq 0 ] && echo -e "\033[0;31m✗\033[0m $file"
            echo "  Missing newline at end of file"
            has_issue=1
        fi

        [ $has_issue -eq 0 ] && echo -e "\033[0;32m✓\033[0m $file"
        exit $has_issue
    ' _ {} \;
}

# Usage:
# lineguard_check "*.txt"      # Check .txt files in current directory
# lineguard_check "*.rs" -r    # Recursively check .rs files
```

### Comparison

| Feature | LineGuard | Shell Commands |
|---------|-----------|----------------|
| Performance | Fast (parallel Rust) | Slower (sequential bash) |
| Binary file detection | Automatic | Manual with `file` command |
| Progress bar | Yes (for large sets) | No |
| Configuration file | Yes (.lineguardrc) | No |
| Output formats | Human, JSON, GitHub | Basic text only |
| Cross-platform | Yes | Unix/Linux/macOS only |
| Memory usage | Efficient streaming | Depends on file size |
| Error handling | Comprehensive | Basic |
| Installation | Required | Uses built-in tools |

Choose LineGuard for production use and comprehensive checking. Use shell commands for quick checks or when installation is not possible.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

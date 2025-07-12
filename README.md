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

```bash
cargo test
```

### Code Quality

```bash
cargo fmt      # Format code
cargo clippy   # Run linter
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
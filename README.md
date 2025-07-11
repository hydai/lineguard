# LineGuard

A fast and reliable file linter that ensures proper line endings and clean formatting.

## Features

- ✅ **Newline Ending Check**: Ensures files end with exactly one newline character
- ✅ **Trailing Space Detection**: Identifies and reports trailing whitespace at line ends
- 🚀 **High Performance**: Parallel file processing for speed
- 🎨 **Multiple Output Formats**: Human-readable, JSON, and GitHub Actions formats
- 🔧 **Configurable**: Flexible configuration via CLI flags or config files

## Installation

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
      --stdin                  Read file paths from stdin
      --no-newline-check       Disable newline ending check
      --no-trailing-space      Disable trailing space check
  -h, --help                   Print help
  -V, --version                Print version
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

**GitHub Actions Format**
```bash
lineguard --format github src/
```

## Exit Codes

- `0` - Success, no issues found
- `1` - Issues found in checked files
- `2` - Error in command-line arguments
- `3` - File access or I/O error
- `4` - Configuration error

## Development

This project follows strict Test-Driven Development (TDD) practices. See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for development guidelines.

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
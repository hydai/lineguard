# LineLint Technical Architecture

## Overview
LineLint follows a modular architecture with clear separation of concerns, making it easy to test, maintain, and extend.

## Core Components

### 1. CLI Module (`cli.rs`)
- **Responsibility**: Command-line argument parsing and validation
- **Key Features**:
  - Uses `clap` crate for argument parsing
  - Validates input parameters
  - Handles help and version display
- **Interfaces**:
  - `parse_args() -> CliArgs`
  - `validate_args(args: &CliArgs) -> Result<(), Error>`

### 2. File Discovery Module (`discovery.rs`)
- **Responsibility**: Finding files to check based on input patterns
- **Key Features**:
  - Glob pattern expansion
  - Recursive directory traversal
  - Stdin file list processing
  - Ignore pattern filtering
- **Interfaces**:
  - `discover_files(args: &CliArgs) -> Result<Vec<PathBuf>, Error>`
  - `should_check_file(path: &Path, config: &Config) -> bool`

### 3. Checker Module (`checker.rs`)
- **Responsibility**: Core lint checking logic
- **Key Features**:
  - Newline ending validation
  - Trailing space detection
  - Parallel file processing
- **Interfaces**:
  - `check_file(path: &Path) -> CheckResult`
  - `check_newline_ending(content: &str) -> Option<Issue>`
  - `check_trailing_spaces(content: &str) -> Vec<Issue>`

### 4. Reporter Module (`reporter.rs`)
- **Responsibility**: Formatting and outputting results
- **Key Features**:
  - Multiple output formats (human, JSON, GitHub)
  - Colored output support
  - Progress indication
- **Interfaces**:
  - `trait Reporter { fn report(&self, results: &[CheckResult]); }`
  - `create_reporter(format: OutputFormat) -> Box<dyn Reporter>`

### 5. Configuration Module (`config.rs`)
- **Responsibility**: Loading and managing configuration
- **Key Features**:
  - Config file parsing (`.linelintrc`)
  - Default configuration
  - Environment variable support
- **Interfaces**:
  - `load_config(path: Option<&Path>) -> Result<Config, Error>`
  - `merge_cli_config(config: Config, args: &CliArgs) -> Config`

## Data Structures

### Core Types
```rust
pub struct CliArgs {
    pub files: Vec<String>,
    pub stdin: bool,
    pub recursive: bool,
    pub format: OutputFormat,
    pub quiet: bool,
    pub verbose: bool,
    pub no_color: bool,
    pub config: Option<PathBuf>,
    pub ignore: Vec<String>,
    pub extensions: Option<Vec<String>>,
    pub no_newline_check: bool,
    pub no_trailing_space: bool,
}

pub struct Config {
    pub checks: CheckConfig,
    pub ignore_patterns: Vec<String>,
    pub file_extensions: Vec<String>,
}

pub struct CheckConfig {
    pub newline_ending: bool,
    pub trailing_spaces: bool,
}

pub struct CheckResult {
    pub file_path: PathBuf,
    pub issues: Vec<Issue>,
}

pub struct Issue {
    pub issue_type: IssueType,
    pub line: Option<usize>,
    pub message: String,
}

pub enum IssueType {
    MissingNewline,
    MultipleNewlines,
    TrailingSpace,
}

pub enum OutputFormat {
    Human,
    Json,
    GitHub,
}
```

## Dependencies

### External Crates
- `clap` (v4) - Command-line argument parsing
- `glob` - File pattern matching
- `serde` / `serde_json` - JSON serialization
- `colored` - Terminal color output
- `rayon` - Parallel processing
- `anyhow` - Error handling
- `thiserror` - Error type definitions
- `indicatif` - Progress bars

### Testing Dependencies
- `assert_cmd` - CLI testing
- `predicates` - Test assertions
- `tempfile` - Temporary test files

## Error Handling

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum LineLintError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Pattern error: {0}")]
    Pattern(#[from] glob::PatternError),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("No files found matching pattern")]
    NoFilesFound,
}
```

## Performance Considerations

### Parallel Processing
- Use `rayon` for parallel file checking
- Configurable thread pool size
- Batch processing for small files

### Memory Efficiency
- Stream large files line-by-line
- Avoid loading entire file content when possible
- Use `BufReader` for file operations

### Optimization Strategies
- Skip binary file detection
- Cache compiled regex patterns
- Early exit on first issue (when appropriate)

## Testing Strategy

### Unit Tests
- Each module has corresponding test module
- Test individual functions in isolation
- Mock file system operations where needed

### Integration Tests
- Test CLI with actual file operations
- Test different output formats
- Test error scenarios

### Test Organization
```
tests/
├── cli_tests.rs         # CLI argument handling
├── checker_tests.rs     # Core checking logic
├── reporter_tests.rs    # Output formatting
└── integration_tests.rs # End-to-end scenarios
```

## Build Configuration

### Cargo.toml Structure
```toml
[package]
name = "linelint"
version = "0.1.0"
edition = "2021"

[dependencies]
# Listed above

[dev-dependencies]
# Testing dependencies

[profile.release]
lto = true
codegen-units = 1
opt-level = 3
```

## Module Dependency Graph
```
main.rs
   ├── cli.rs
   ├── config.rs
   ├── discovery.rs
   │     └── config.rs
   ├── checker.rs
   └── reporter.rs
         └── checker.rs (for types)
```
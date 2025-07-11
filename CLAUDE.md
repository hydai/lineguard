# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

LineLint is a Rust-based command-line tool that validates text files for proper line endings and formatting. It checks that files end with a newline character and flags trailing spaces at the end of lines.

## Development Workflow - STRICT TDD REQUIRED

Every development cycle MUST follow this exact sequence:

1. **Write Minimal Test First** - Create the smallest test that meets requirements (must fail initially)
2. **Pass the Test** - Write minimal code to make test pass
3. **Git Commit** - Only commit AFTER tests pass
4. **Refactor** - Only allowed AFTER git commit
5. **Quality Checks** - Run ALL of these after each implementation:
   ```bash
   cargo fmt --all                                # Format code
   cargo clippy --all-targets --all-features -- -D warnings  # Run linter
   cargo build                                    # Ensure compilation
   cargo test                                     # Run all tests
   ```

## Essential Commands

```bash
# Development
cargo build              # Build the project
cargo run -- [args]      # Run with arguments
cargo test              # Run all tests
cargo test [test_name]  # Run specific test

# Quality Checks (MUST run before commits)
cargo fmt                                      # Format code
cargo clippy --all-targets --all-features -- -D warnings  # Lint code
cargo build                                    # Check compilation

# Running the tool
cargo run -- file.txt                    # Check single file
cargo run -- "src/**/*.rs"               # Check with glob pattern
cargo run -- --format json file.txt      # Output as JSON
```

## Architecture Overview

The codebase is organized into these core modules:

- **cli.rs** - Command-line parsing with `clap`
- **discovery.rs** - File discovery and glob pattern handling
- **checker.rs** - Core lint checking logic (newline and trailing space detection)
- **reporter.rs** - Output formatting (human, JSON, GitHub Actions formats)
- **config.rs** - Configuration file handling

## Key Implementation Notes

### Exit Codes
- 0: Success (no issues found)
- 1: Issues found in checked files
- 2: Invalid command-line arguments
- 3: File I/O error
- 4: Configuration error

### Performance Requirements
- Must handle 10,000 files in under 5 seconds
- Use `rayon` for parallel processing
- Stream large files instead of loading into memory

### Testing Structure
```
tests/
├── cli_tests.rs         # CLI argument handling
├── checker_tests.rs     # Core checking logic
├── reporter_tests.rs    # Output formatting
└── integration_tests.rs # End-to-end scenarios
```

## Current Implementation Status

The project is in the planning phase with complete specifications. Implementation follows the plan in IMPLEMENTATION_PLAN.md with 6 phases:
1. Project Setup
2. File Discovery and CLI
3. Core Checking Logic
4. Output and Reporting
5. Advanced Features
6. Polish and Release

## Critical Reminders

- NEVER skip the TDD cycle - test first, then implement
- ALWAYS run quality checks before committing
- Each commit should represent one completed feature with passing tests
- Refactoring is only allowed after committing working code
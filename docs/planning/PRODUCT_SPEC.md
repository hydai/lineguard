# LineGuard - Product Specification

## Overview
LineGuard is a command-line tool written in Rust that validates text files for proper line endings and formatting. It ensures code quality by checking that files end with a newline character and flags any trailing spaces at the end of lines.

## Core Features

### 1. Newline Ending Check
- Validates that files end with exactly one newline character (`\n`)
- Reports files missing the final newline
- Reports files with multiple trailing newlines

### 2. Trailing Space Detection
- Scans each line for trailing whitespace characters
- Reports line numbers with trailing spaces
- Supports both space and tab characters as trailing whitespace

## User Stories

1. **As a developer**, I want to check all files in my project for proper line endings so that my code follows best practices.

2. **As a CI/CD engineer**, I want to integrate lineguard into my pipeline to automatically catch formatting issues before merging code.

3. **As a team lead**, I want to enforce consistent file formatting across my team's codebase.

4. **As a developer**, I want to automatically fix formatting issues instead of manually correcting them.

## Functional Requirements

### Input
- Accept file paths as command-line arguments
- Support glob patterns (e.g., `*.rs`, `**/*.txt`)
- Support reading file list from stdin
- Support recursive directory scanning

### Output
- Clear, actionable error messages
- Exit codes: 0 for success, non-zero for violations found
- Support different output formats (human-readable, JSON, GitHub Actions)
- Summary statistics (files checked, violations found)

### Configuration
- Support `.lineguardrc` configuration file
- Allow enabling/disabling specific checks
- Support ignore patterns (similar to .gitignore)
- Configurable file extensions to check
- Auto-discovery of config files in parent directories
- CLI flags override configuration file settings

## Non-Functional Requirements

### Performance
- Process files in parallel for faster execution
- Stream large files instead of loading entirely into memory
- Target: Check 10,000 files in under 5 seconds

### Usability
- Clear help documentation
- Intuitive command-line interface
- Colored output for better readability
- Progress indicator for large file sets

### Reliability
- Graceful handling of binary files
- Proper error handling for permission issues
- Support for different line ending styles (LF, CRLF)

## Development Process Requirements

### Test-Driven Development (TDD)
Every development cycle MUST strictly follow:

1. **Write Minimal Test First**
   - Create the smallest, most concise test that meets the task requirements
   - Test must be focused on a single behavior
   - Test must fail initially (Red phase)

2. **Pass the Test**
   - Write minimal code to make the test pass
   - No additional functionality beyond what's tested
   - Verify test passes (Green phase)

3. **Git Commit**
   - Only commit AFTER tests are passing
   - Commit message should describe the functionality added
   - Include test files in the commit

4. **Refactor**
   - Refactoring is only allowed AFTER git commit
   - Maintain all tests in passing state
   - Improve code quality without changing behavior

5. **Quality Checks**
   - After each implementation, MUST run:
     - `cargo fmt` - Rust formatter
     - `cargo clippy` - Rust linter
     - `cargo build` - Compilation check
   - Fix any issues before proceeding

### Development Workflow
```
1. Write test → 2. Run test (fails) → 3. Implement → 4. Run test (passes) → 
5. Run quality checks → 6. Git commit → 7. Refactor (if needed) → 8. Run all checks
```

## Success Metrics
- Execution speed compared to similar tools
- User adoption rate
- Integration with popular CI/CD platforms
- Community contributions and feedback
- 100% test coverage for core functionality

## MVP Scope
For the initial release, focus on:
1. Basic CLI with file path arguments
2. Newline ending check
3. Trailing space detection
4. Human-readable output format
5. Exit codes for CI/CD integration

## Implemented Features
All planned features have been successfully implemented:
- ✅ Configuration file support (`.lineguardrc`)
- ✅ Multiple output formats (human, JSON, GitHub Actions)
- ✅ Auto-fix capability (`--fix` and `--dry-run` flags)
- ✅ Streaming support for large files
- ✅ Permission error handling
- ✅ CLI flags to disable specific checks
- ✅ Ignore patterns and file extension filtering
- ✅ Progress indicators for large file sets
- ✅ Parallel processing with rayon
- ✅ Binary file detection and skipping
- ✅ Git commit range filtering (`--from` and `--to` flags)

## Future Enhancements
- Editor integrations (VS Code, Vim, etc.)
- Additional lint rules (indentation, line length, etc.)
- Git pre-commit hook integration
- Performance profiling and optimization
- Internationalization support
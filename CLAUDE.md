# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

LineGuard is a Rust command-line linter that checks text files for a single trailing newline and for trailing whitespace, and can fix both with `--fix`. The crate builds a library (`lineguard`) plus the `lineguard` binary; `src/main.rs` only wires the library modules together. The feature set in docs/planning/PRODUCT_SPEC.md is implemented; the other files under docs/planning are historical.

## Development Workflow - STRICT TDD REQUIRED

Every development cycle MUST follow this exact sequence:

1. **Write Minimal Test First** - Create the smallest test that meets requirements (must fail initially)
2. **Pass the Test** - Write minimal code to make test pass
3. **Git Commit** - Only commit AFTER tests pass
4. **Refactor** - Only allowed AFTER git commit
5. **Quality Checks** - Run ALL of these after each implementation (CI runs the same commands):
   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items
   cargo build
   cargo test
   ```

`make check` runs fmt, clippy, build and test in one go.

## Essential Commands

```bash
cargo build                              # Build
cargo test                               # All tests (unit + integration)
cargo test --lib                         # Unit tests only (fast)
cargo test --test fix_tests              # One integration test binary
cargo test <name>                        # Tests matching a name

cargo run -- file.txt                    # Check a file
cargo run -- -r src/                     # Check a directory recursively
cargo run -- --format json "src/**/*.rs" # JSON output
cargo run -- --fix --dry-run .           # Preview fixes
cargo run -- --from HEAD~1 .             # Only files changed since a commit
```

## Architecture

```
src/main.rs      parse args -> load config -> discover files -> rayon par_iter check/fix -> report -> exit code
src/cli/         CliArgs (clap derive), OutputFormat
src/config/      Config and CheckConfig; .lineguardrc is found by walking up from the current directory
src/discovery/   paths, globs, directories, --stdin, --ignore/--extensions, --no-hidden, git range filter
src/checker/     CheckerCore (pure checks on &str), FileChecker<R: FileReader> (in-memory vs streaming), result types
src/fixer/       fix_file: rewrites files, via a temp file plus rename for large files
src/git/         git diff --name-only wrapper behind --from/--to
src/reporter/    Reporter trait; human, json and github implementations; Output traits used by tests
src/testing/     #[cfg(test)] only: MockFileSystem, MockOutput, TestFileBuilder
```

Details: docs/developer/ARCHITECTURE.md and docs/developer/ERROR_HANDLING.md.

## Key Implementation Notes

- **Exit codes**: 0 clean, 1 issues found (or a fix failed), 2 bad arguments (clap), 3 discovery failed (stdin or git), 4 configuration error. A file that cannot be read is reported on stderr and in the JSON `errors` array but does not change the exit code.
- **CLI overrides config**: `--ignore` and `--extensions` replace (not extend) the values from `.lineguardrc`; `--no-newline-check` and `--no-trailing-space` switch checks off.
- **Streaming threshold**: files larger than 10MB (10 * 1024 * 1024 bytes) take the streaming code paths in both the checker and the fixer. Unit tests select that path by faking the size with `MockFileSystem::set_metadata`.
- **Performance target**: 10,000 files in under 5 seconds; keep file checks parallel (rayon) and never load large files whole.
- **Line endings**: the checker works on `str::lines()` output, so CRLF files are accepted; keep new code CRLF-safe.

## Testing

- Unit tests live in `#[cfg(test)] mod tests` next to the code and use the helpers in `src/testing`, which are not available to integration tests or other crates.
- Integration tests in `tests/` run the built binary with `assert_cmd` inside a `tempfile::TempDir` and assert on exit code and output.
- CI (`.github/workflows/ci.yml`) only triggers on changes under `src/**` and `Cargo.*`; a tests-only change gets no CI run, so run the full local checks yourself.

## Releases

Releases are automated with knope from conventional commits: `feat:` and `fix:` commits on master make `prepare-release.yml` open a "chore: prepare release" PR, and merging that PR builds the binaries, publishes to crates.io and creates the GitHub release. Never edit `CHANGELOG.md` or the version in `Cargo.toml` by hand.

## Critical Reminders

- NEVER skip the TDD cycle - test first, then implement
- ALWAYS run the quality checks above before committing
- Each commit should represent one completed change with passing tests
- Refactoring is only allowed after committing working code

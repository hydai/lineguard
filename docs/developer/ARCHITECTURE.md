# LineGuard Architecture

## Overview

LineGuard is one crate that produces a library (`lineguard`) and a binary of
the same name. `src/main.rs` is a thin driver: it parses the command line,
loads the configuration, discovers files, checks or fixes them in parallel and
prints a report. Everything reusable lives in the library modules under `src/`.

## Module Layout

```
src/
├── main.rs              Binary entry point: wiring, progress bar, exit codes
├── lib.rs               Module declarations and re-exports
├── cli/mod.rs           clap definition of CliArgs and OutputFormat
├── config/mod.rs        Config, CheckConfig, .lineguardrc lookup and parsing
├── discovery/mod.rs     Turns paths, globs, directories and stdin into a file list
├── checker/
│   ├── mod.rs           CheckResult, Issue, IssueType and the check_file entry point
│   ├── core.rs          CheckerCore: pure content checks (trailing whitespace, final newline)
│   ├── file_checker.rs  FileChecker<R: FileReader>: in-memory and streaming file checks
│   └── io_trait.rs      FileReader trait and FileMetadata (I/O abstraction for tests)
├── fixer/mod.rs         fix_file: rewrites a file to remove the reported issues
├── git/mod.rs           git wrapper for --from/--to (changed-file listing)
├── reporter/
│   ├── mod.rs           Reporter trait and re-exports
│   ├── traits.rs        Output and ColoredOutput traits, StdOutput, ReporterWithOutput
│   ├── human.rs         HumanReporter (default format, optional colors)
│   ├── json.rs          JsonReporter
│   └── github.rs        GitHubReporter (::error annotations)
└── testing/             #[cfg(test)] only: MockFileSystem, MockOutput, TestFileBuilder
```

## Data Flow

1. `cli::parse_args` builds a `CliArgs` (clap derive).
2. `config::load_config` reads the file given with `--config`; without it, the
   first `.lineguardrc` found while walking up from the current directory;
   without one, `Config::default()`. `--no-newline-check` and
   `--no-trailing-space` are then applied on top of the loaded config.
3. `discovery::discover_files` merges `--ignore` and `--extensions` over the
   config (CLI wins), expands each argument (directory, glob, literal path) or
   reads paths from stdin with `--stdin`, drops files with binary extensions,
   ignored paths and, with `--no-hidden`, dotfiles. Directories are walked with
   the `ignore` crate: inside a git repository, `.gitignore`,
   `.git/info/exclude` and the `.git` directory itself are respected/skipped
   unless `--no-gitignore` (or `respect_gitignore = false` in the config)
   disables it; explicitly named files, globs and stdin paths are never
   gitignore-filtered. With `--from`, only files
   that `git::get_changed_files` lists as changed between the two commits are
   kept. The result is a `DiscoveryResult` with the file list and an optional
   `GitRangeInfo` that `--verbose` prints.
4. `main` maps `checker::check_file` over the files with rayon's `par_iter`.
   Files larger than 10MB go through `FileChecker::check_file_streaming`;
   everything else is read into memory and handed to
   `CheckerCore::check_content`.
5. In `--fix` mode every file with issues goes to `fixer::fix_file`
   (`--dry-run` only reports). Otherwise the results go to the reporter
   selected by `--format`.
6. The exit code is derived from the results (see Exit Codes).

## Core Types

```rust
// cli
pub struct CliArgs {
    files, stdin, recursive, format, quiet, verbose, no_color, config,
    ignore, extensions, no_newline_check, no_trailing_space, fix, dry_run,
    from, to, no_hidden, no_gitignore,
}
pub enum OutputFormat { Human, Json, GitHub }

// config
pub struct Config { checks: CheckConfig, ignore_patterns: Vec<String>, file_extensions: Vec<String>, respect_gitignore: bool }
pub struct CheckConfig { newline_ending: bool, trailing_spaces: bool } // both default to true

// discovery
pub struct DiscoveryResult { files: Vec<PathBuf>, git_range: Option<GitRangeInfo> }
pub struct GitRangeInfo { from: String, to: String, changed_files: Vec<PathBuf> }

// checker
pub struct CheckResult { file_path: PathBuf, issues: Vec<Issue>, error: Option<String> }
pub struct Issue { issue_type: IssueType, line: Option<usize>, message: String }
pub enum IssueType { MissingNewline, MultipleNewlines, TrailingSpace }

// fixer
pub struct FixResult { file_path: PathBuf, fixed: bool, issues_fixed: Vec<Issue> }
```

## Checking Rules

- **Trailing whitespace**: a line is reported when `line.trim_end()` is shorter
  than the line, so spaces, tabs and other Unicode whitespace all count. Line
  numbers are 1-based.
- **Final newline**: an empty file is accepted. Any other file must end with
  exactly one line ending: no `\n` at the end is `MissingNewline`, a blank line
  at the end is `MultipleNewlines`.
- Each rule can be switched off in the `[checks]` table of `.lineguardrc` or
  with the matching CLI flag.

## Large Files

Files larger than 10MB (10 × 1024 × 1024 bytes) are never loaded whole. The
checker reads them line by line and inspects the last bytes of the file for
the final-newline rule. The fixer writes the corrected content to `<file>.tmp`
next to the original and renames it over the original when done.

## Reporters

`Reporter::report(&[CheckResult])` prints to stdout and is what `main` calls.
Every reporter also implements `ReporterWithOutput::report_to`, which writes
to any `Output`; the unit tests pass a `MockOutput` to capture the text.

- **human**: one block per file with issues, then `✓ All files passed lint
  checks!` or `✗ Found N issues in M files`, followed by `Files checked: X`.
  Colors come from the `colored` crate unless `--no-color` is given.
- **json**: `files_checked`, `files_with_issues`, `total_issues`, `issues`
  (per file: `type`, `line`, `message`) and `errors` when some file could not
  be read.
- **github**: `::error file=<path>[,line=<n>]::<message>` for each issue and
  for each read error.

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | No issues (also when no files matched) |
| 1 | Issues found, or in `--fix` mode a file could not be fixed |
| 2 | Invalid command-line usage (reported by clap) |
| 3 | File discovery failed: stdin could not be read, or `--from`/`--to` could not be resolved |
| 4 | Configuration file missing or not valid TOML |

Files that cannot be read are reported on stderr, in the JSON `errors` array
and as GitHub annotations, but they do not change the exit code.

## Dependencies

| Crate | Role |
|-------|------|
| clap | Command-line parsing |
| glob | Pattern expansion and ignore matching |
| rayon | Parallel file checking |
| indicatif | Progress bar (human format, more than 10 files) |
| colored | Terminal colors |
| serde, toml | `.lineguardrc` parsing |
| serde_json | JSON output |
| anyhow | Error propagation |

Development: `assert_cmd` and `predicates` drive the binary in integration
tests, `tempfile` provides scratch directories.

## Testing

Unit tests sit next to the code in `#[cfg(test)] mod tests` blocks and use
the helpers in `src/testing`, which only exist in test builds:

- `MockFileSystem` implements `FileReader`, so `FileChecker` can be exercised
  without touching the disk. Faking the file size selects the streaming path.
- `MockOutput` implements `Output` and `ColoredOutput` and records what a
  reporter wrote.
- `TestFileBuilder` assembles file contents with trailing spaces, tabs, CRLF
  endings or a missing final newline.

Integration tests under `tests/` run the built binary with `assert_cmd`
against temporary directories and check exit codes and output.

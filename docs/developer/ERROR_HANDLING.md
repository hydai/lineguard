# LineGuard Error Handling

This document describes how the current implementation reports problems.
Lint findings are not errors: they go to stdout in the chosen format and are
described in the [CLI reference](../user/CLI_REFERENCE.md).

## Principles

- A problem with one file never stops the run. Read errors are stored in
  `CheckResult::error`, the remaining files are still checked.
- Problems that make the whole run meaningless (bad configuration, bad
  arguments, git failures) abort with a distinct exit code.
- Error messages go to stderr. The JSON and GitHub formats additionally carry
  per-file read errors in their stdout output so CI consumers see them.

## Exit Codes

| Code | When | Decided in |
|------|------|------------|
| 0 | No lint issues. Also when no files matched, and when unreadable files were the only problem. | `main.rs` |
| 1 | At least one lint issue. In `--fix` mode: at least one file could not be fixed. | `main.rs` |
| 2 | Invalid command-line usage (unknown flag, bad value). | clap |
| 3 | File discovery failed: stdin could not be read, or `--from`/`--to` could not be resolved (not a git repository, unknown reference, git unavailable). | `main.rs`, after `discover_files` |
| 4 | Configuration could not be loaded: the `--config` path does not exist, cannot be read, or is not valid TOML for `Config`. | `main.rs`, after `load_config` |

## Messages by Stage

### Configuration

```
Error loading configuration: Configuration file not found: <path>
Error loading configuration: <TOML parse error>
```

Printed to stderr, exit code 4. Unknown keys in `.lineguardrc` are ignored;
missing keys take their defaults.

### Discovery

- A directory that cannot be listed prints `<dir>: <os error>` and is skipped;
  the run continues.
- A directory entry that cannot be read prints
  `Error reading directory entry: <error>` and is skipped.
- A stdin read failure or a git failure prints `Error: <details>` and exits
  with code 3. Git failures look like `not a git repository`,
  `Invalid git reference: <ref>: <git stderr>` or
  `Failed to get changed files: <git stderr>`.
- When no file is left after filtering, `No files found to check` is printed
  to stderr (suppressed by `--quiet`) and the exit code is 0.

### Checking

A file whose metadata or content cannot be read (missing file, permission
denied, content that is not valid UTF-8) yields a `CheckResult` with
`error = "<path>: <io error>"` and no issues. That text is:

- printed to stderr as is, unless `--quiet` is given;
- added to the `errors` array of the JSON output;
- emitted as `::error file=<path>::<io error>` by the GitHub format.

The exit code is not affected by read errors.

Known gap: in streaming mode (files over 10MB) a line that cannot be decoded
stops the scan of that file without recording an error.

### Fixing

With `--fix`, every file whose check reported issues is rewritten. In the
human format each rewritten file prints `Fixed: <path>` (or `Would fix:
<path>` with `--dry-run`), followed by a summary line `Fixed N file(s)`. A
file that cannot be rewritten (for example a read-only file, or a failed
rename of the temporary file) prints `<path>: <error>` to stderr, the run ends
with `N error(s) occurred` on stderr and exit code 1.

`--quiet` suppresses all fix-mode output. The JSON and GitHub formats print
nothing in fix mode; this is a known gap.

## Where Errors Are Represented

| Location | Content |
|----------|---------|
| `CheckResult::error: Option<String>` | Per-file read error, formatted as `<path>: <io error>` |
| `fixer::fix_file` return value | `Result<FixResult, anyhow::Error>` per file |
| `discovery::discover_files` return value | `Result<DiscoveryResult, anyhow::Error>` for the whole run |
| `config::load_config` return value | `Result<Config, anyhow::Error>` for the whole run |

All fallible library functions use `anyhow::Error`; there is no custom error
enum.

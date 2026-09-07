# Code Coverage

Coverage is measured with [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
and tracked on [Codecov](https://codecov.io/gh/hydai/lineguard). The
`coverage.yml` workflow uploads two reports for every push to `master` and
every pull request that touches `src/**` or `Cargo.*`: unit tests only
(`--lib`, flag `unit-tests`) and the full suite (`--all`, flag `all-tests`).

## Thresholds

`codecov.yml` is the source of truth:

| Check | Target | Tolerance |
|-------|--------|-----------|
| Project (whole codebase) | 75% | 2% |
| Patch (lines changed by a PR) | 70% | 3% |

Test code (`tests/`, `**/tests.rs`, `**/*test*`) is excluded from the
measurement. Treat the thresholds as a floor: the core modules (`checker`,
`reporter`, `fixer`, `discovery`) should stay well above them.

## Running Coverage Locally

Install the tool once:

```bash
cargo install cargo-tarpaulin
```

Then:

```bash
# Unit tests only (fast)
cargo tarpaulin --lib --print-summary

# All tests
cargo tarpaulin --all --print-summary

# HTML report (written to tarpaulin-report.html)
cargo tarpaulin --lib --out html

# Cobertura XML, as produced in CI
cargo tarpaulin --all --out xml --avoid-cfg-tarpaulin
```

`make coverage` runs the HTML variant.

## What to Test

**Must test**
- Public API functions
- Error handling paths
- Edge cases and boundaries (empty files, files without a final newline, CRLF)
- Configuration parsing
- File I/O through the `FileReader` mock

**Can skip**
- Simple getters and derived trait implementations
- Panic branches for unreachable states
- Terminal capability detection inside third-party crates

## Writing Testable Code

The codebase separates I/O from logic so both can be covered by unit tests:

- `CheckerCore` works on `&str` content and has no I/O.
- `FileChecker<R: FileReader>` takes the file access as a type parameter;
  `MockFileSystem` implements `FileReader` for tests and can fake the file
  size to exercise the streaming path.
- Reporters implement `ReporterWithOutput::report_to(&self, results, &mut dyn Output)`;
  `MockOutput` captures the text in tests.

When adding code, keep that split: put the decision in a function that takes
plain data, and keep the file or terminal access in a thin wrapper around it.

## Known Gap

`FileChecker::check_final_newline_streaming` opens the file with `std::fs`
directly instead of going through `FileReader`, so the streaming final-newline
check is only covered by tests that write real temporary files.

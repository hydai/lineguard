# Changelog
## 0.2.1 (2026-09-09)

### Features

- respect .gitignore by default when scanning directories (#143)

## 0.2.0 (2026-09-07)

### Breaking Changes

- remove the `lineguard::checker::LineChecker` and `lineguard::checker::ContentChecker` traits from the public API; nothing in the library accepted or returned them, so implementors gained nothing from them (#129)

### Fixes

- detect multiple trailing newlines in CRLF files (#130)
- preserve CRLF line endings when fixing files (#131)

## 0.1.8 (2026-09-07)

### Fixes

- update dependencies to address RUSTSEC-2026-0204 (#124)

## 0.1.7 (2026-02-09)

### Features

- migrate release workflow to knope
- migrate crates.io publishing to OIDC trusted publishing

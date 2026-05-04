# GitHub Release Packaging

This document defines the first-class release process for RustPluginSuite.

## Trigger model

Releases are triggered when a tag matching `v*` is pushed, e.g. `v0.1.0`.

Workflow: `.github/workflows/release.yml`

## Pipeline stages

1. **Test matrix** (`ubuntu-latest`, `windows-latest`)
   - `cargo fmt --all -- --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test --all-targets --all-features`
2. **Package** (`ubuntu-latest`)
   - runs `scripts/release/package-release.sh`
   - uploads generated files from `dist/`
3. **Publish** (tag builds only)
   - creates GitHub release
   - attaches all `dist/*` assets
   - applies release body template from `.github/release/release-notes-template.md`

## Packaged contents

Generated bundle includes:

- `Cargo.toml`
- `Cargo.lock`
- `README.md`
- `docs/`
- `scripts/`
- `src/`
- `tests/`
- `windows-installer/install-windows-bootstrap.ps1`
- `windows-installer/install-windows.ps1`
- `windows-installer/install-windows.bat`
- `RELEASE-MANIFEST.txt`

## Local dry-run

```bash
bash scripts/release/package-release.sh v0.1.0
ls -la dist/
```

## Integrity verification

A `*-SHA256SUMS.txt` file is produced with SHA-256 checksums for `.zip` and `.tar.gz` artifacts.

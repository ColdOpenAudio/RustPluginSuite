# Windows Auto Installation Wrapper

RustPluginSuite now includes a production-oriented Windows installation wrapper.

## Files

- `scripts/install-windows.bat` — thin launcher for users who double-click a batch file.
- `scripts/install-windows.ps1` — full installer logic.

## What the installer does

1. Validates required tooling (`git`, PowerShell).
2. Ensures Rust is installed (prefers `winget`, then `choco`, then direct `rustup-init.exe`).
3. Pins/initializes stable toolchain.
4. Runs quality gates:
   - `cargo fmt -- --check`
   - `cargo test`
5. Builds release artifacts (`cargo build --release`) unless `-SkipBuild` is passed.
6. Deploys output and metadata to install directory (default: `C:\Program Files\RustPluginSuite`).

## Usage

### Explorer / CMD

```bat
scripts\install-windows.bat
```

### PowerShell (default install)

```powershell
.\scripts\install-windows.ps1
```

### PowerShell (custom install directory)

```powershell
.\scripts\install-windows.ps1 -InstallDir "C:\Audio\RustPluginSuite"
```

### PowerShell (verification-only mode)

```powershell
.\scripts\install-windows.ps1 -SkipBuild
```

## Deployment outputs

The installer writes the following into the destination directory:

- `README.md`
- any `.dll` files produced in `target\release`
- `install-manifest.json` (timestamp + deployment metadata)

## Operational notes

- Running under elevated PowerShell is recommended for writes to `C:\Program Files`.
- Installer is idempotent and safe to rerun.
- The wrapper intentionally fails fast when prerequisite checks or test/build steps fail.


## EXE packaging

If you need an executable wrapper for distribution, package `scripts/install-windows.bat` and `scripts/install-windows.ps1` together inside a signed installer (for example MSI/EXE built by your release tooling).

Recommended hardening for enterprise distribution:

- code-sign the EXE wrapper and PowerShell payload
- publish SHA-256 checksums with each release
- keep SmartScreen reputation by shipping consistent signed publisher metadata
- never disguise the installer as a game cheat, crack, or deceptive utility

This project only supports legitimate installation and maintenance workflows.

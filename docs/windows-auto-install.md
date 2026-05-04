# Windows Auto Installation Wrapper — Technical Execution Map

This document describes the Windows installer as an execution topology (entry points, control flow, fallback branches, and deployment side effects), not just a feature summary.

## 1) Runtime Topology (Component Graph)

### 1.1 Entry Nodes

- **`scripts/install-windows.bat`**
  - Role: compatibility launcher for Explorer double-click and legacy `cmd.exe` workflows.
  - Behavior: locates `pwsh.exe`, falls back to `powershell.exe`, and can use `winget` to install PowerShell 7 if no PowerShell host is discoverable.
- **`scripts/install-windows-bootstrap.ps1`**
  - Role: host/toolchain bootstrapper.
  - Behavior: ensures PowerShell 7, re-enters itself under `pwsh.exe`, verifies Git, installs Rust when needed, and delegates to the product installer.
- **`scripts/install-windows.ps1`**
  - Role: authoritative product installation engine.
  - Behavior: verifies already-bootstrapped prerequisites, runs quality gates, builds, deploys, and emits the install manifest.
- **`scripts/setup/install-git-windows.ps1`**
  - Role: standalone Git installer helper.
  - Behavior: installs Git through `winget`, Chocolatey, or the official Git for Windows installer.
- **`scripts/setup/install-rust-windows.ps1`**
  - Role: standalone Rust installer helper.
  - Behavior: installs Rust through `winget`, Chocolatey, or `rustup-init.exe`.

### 1.2 Execution Domains

- **Host domain**: Windows shell environment (`cmd.exe` or PowerShell).
- **Tooling domain**: `git`, PowerShell runtime, Rust toolchain (`rustup`, `cargo`, `rustc`).
- **Build domain**: Cargo workspace compilation/tests in repository context.
- **Deployment domain**: filesystem writes to install target (default `C:\Program Files\RustPluginSuite`).

## 2) Control Flow Map (Ordered Pipeline)

### Stage A — Batch Launcher Host Selection

1. Resolve `install-windows-bootstrap.ps1` next to the batch file.
2. Prefer `pwsh.exe` if it is already on `PATH`.
3. Fall back to Windows PowerShell (`powershell.exe`) when `pwsh.exe` is unavailable.
4. If neither shell is discoverable, try `winget install --id Microsoft.PowerShell --exact --source winget`.
5. Launch the bootstrap script with the original user arguments.

**Exit condition:** hard-fail with manual `winget` instructions if no usable PowerShell host can be found or installed.

### Stage B — PowerShell 7 Bootstrap

1. Locate `pwsh.exe` on `PATH`.
2. Probe standard PowerShell 7 install paths:
   - `C:\Program Files\PowerShell\7\pwsh.exe`
   - `C:\Program Files (x86)\PowerShell\7\pwsh.exe`
3. If missing, attempt acquisition:
   - `winget install --id Microsoft.PowerShell --exact --source winget`
   - `choco install powershell-core -y`
4. If the bootstrap is running under Windows PowerShell, re-enter the bootstrap under PowerShell 7 before continuing.

**Design property:** Windows PowerShell 5.1 is treated as a compatibility bridge; PowerShell 7 is the preferred execution host for the full installer path.

### Stage C — Rust Toolchain Acquisition (Fallback Ladder)

If Rust is missing, the bootstrap delegates to `scripts\setup\install-rust-windows.ps1`, which traverses a strict fallback chain:

1. Attempt installation via **`winget`**.
2. If unavailable/failing, attempt via **Chocolatey (`choco`)**.
3. If both unavailable/failing, download and run **`rustup-init.exe`** directly.

After acquisition path success:

4. Initialize/pin **stable** toolchain.
5. Verify `cargo` command availability in current execution context.

**Design property:** monotonic fallback; each failed branch advances to a more direct installation path.

### Stage D — Quality Gate Circuit

The product installer runs in repository root, through a deterministic process runner:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all-targets --all-features`

**Circuit behavior:**
- Any non-zero exit terminates pipeline.
- No deployment occurs after gate failure.

### Stage E — Build Circuit

- Default mode: run `cargo build --release --all-targets`.
- Verification-only mode (`-SkipBuild`): bypass release build stage.

### Stage F — Deployment & Artifact Materialization

1. Ensure destination directory exists.
2. Copy `README.md`.
3. Copy all produced `target\release\*.dll` artifacts.
4. Generate `install-manifest.json` with deployment metadata and timestamp.

**Output invariant:** install directory becomes a self-describing runtime package containing binaries and manifest metadata.

**Failure invariant:** a default build must produce at least one Windows `.dll`; otherwise deployment fails instead of writing a misleading manifest-only install.

## 3) Data/Artifact Flow Map

### 3.1 Inputs

- Source tree contents in repository root.
- Active PowerShell host state.
- Active Git/Rust/Cargo toolchain state.
- User options:
  - `-InstallDir <path>`
  - `-SkipBuild`
  - `-ForceRustup` (bootstrap only)

### 3.2 Intermediate State

- PowerShell 7 installation side effects (if PowerShell bootstrap required).
- Rust toolchain installation side effects (if Rust bootstrap required).
- Cargo target outputs in `target\release`.
- Process-level exit code stream from each invoked command.

### 3.3 Outputs

- Install directory contents:
  - `README.md`
  - release `.dll` files
  - `install-manifest.json` with install path, repository path, PowerShell edition/version, and build mode.

## 4) Failure Domain Map

### 4.1 Deterministic Failure Boundaries

The installer intentionally hard-stops on:

- missing prerequisites (`git`, PowerShell runtime)
- unsuccessful PowerShell 7 acquisition when no `pwsh.exe` can be found
- unsuccessful Rust acquisition across all fallback branches
- toolchain initialization failure
- formatting, clippy, or test failures
- release build failure (unless `-SkipBuild`)
- artifact copy/manifest write failure

### 4.2 Non-Goals (by design)

- No best-effort partial install after quality/build failure.
- No silent downgrade of quality gates.
- No continuation after non-zero external process status.

## 5) Operational Modes

### 5.1 Explorer / CMD Mode

From a repository checkout:

```bat
scripts\install-windows.bat
```

From an extracted release bundle:

```bat
windows-installer\install-windows.bat
```

### 5.2 PowerShell Default Install

From a repository checkout:

```powershell
.\scripts\install-windows-bootstrap.ps1
```

From an extracted release bundle:

```powershell
.\windows-installer\install-windows-bootstrap.ps1
```

### 5.3 PowerShell Custom Destination

```powershell
.\scripts\install-windows-bootstrap.ps1 -InstallDir "C:\Audio\RustPluginSuite"
```

### 5.4 Verification-Only (No Release Build)

```powershell
.\scripts\install-windows-bootstrap.ps1 -SkipBuild
```

### 5.5 Product Installer Only (Already Bootstrapped)

```powershell
.\scripts\install-windows.ps1
```

## 6) Security & Privilege Boundary Notes

- Writes to `C:\Program Files` typically require elevated PowerShell.
- Installer is designed to be idempotent and safe to rerun.
- Fail-fast semantics prevent stale or partially validated artifacts from being promoted to install destination.

## 7) Practical Launch Path (Post-Install)

After successful deployment, launch/host behavior depends on the produced plugin binaries and target host application. Use the emitted `install-manifest.json` to confirm timestamp, deployed file set, and destination consistency before first load.

Command sheets are available at:

- `docs/command-sheets/git-commands.md`
- `docs/command-sheets/rust-install-commands.md`

# RustPluginSuite — Phase 1 Stereo/M-S Oscilloscope Core

This repository now includes a realtime-safe oscilloscope processing/rendering core organized around strict input/output scope separation.

## Implemented Phase 1 Surface

- Stereo L/R input processing
- M/S transform support (`M=(L+R)*0.5`, `S=(L-R)*0.5`)
- View modes: `XY`, `POLAR`, `SUM_DIFF`, `LISS_SUM`, `DIFF_ONLY`, `DUAL_TRACE`
- RMS windowed energy metric (fixed-size ring)
- DC tracking and optional DC removal
- Passive lookahead infrastructure (disabled by default)
- Frame contract (`Frame`) from input scope to output scope
- Point contract (`Point`) for renderer consumption

## Module Layout

```text
src/
  lib.rs
  params.rs
  input/
    process.rs
    lookahead.rs
    dc.rs
    rms.rs
    ms.rs
    buffer.rs
  output/
    xy.rs
    polar.rs
    sl_modes.rs
    renderer.rs
    view.rs
  shared/
    frame.rs
```

## Realtime Safety Notes

- Input thread does no per-sample allocation; buffers are preallocated during setup.
- Lookahead uses fixed storage and is pass-through when disabled.
- Output view mapping is DSP-free and visualization-only.
- Noise threshold modifies point intensity only (no sample gating).

## Validation

Run:

```bash
cargo fmt
cargo test
```

## Release = Compile, Test, and Package

When the instruction is **release**, run:

```bash
bash scripts/release/release-gate.sh
```

This enforces formatting, linting, full tests, release compilation, and artifact packaging in a single command.

See `docs/release-process.md` for policy and workflow details.

## Windows Auto-Install Wrapper

A complete Windows installer wrapper is available for one-command setup, validation, build, and deployment:

- `scripts\install-windows.bat` (double-click/CMD entrypoint)
- `scripts\install-windows-bootstrap.ps1` (PowerShell/Git/Rust bootstrapper)
- `scripts\install-windows.ps1` (build/deploy installer)

See `docs/windows-auto-install.md` for full usage and operational details.

## Git + Rust Command Sheets

- `docs/command-sheets/git-commands.md`
- `docs/command-sheets/rust-install-commands.md`

## PFGA (Pitch/Frequency Gate Amp) System Spec

The combined **Pitch/Frequency Gate Amp (PFGA)** architecture and validation protocol are documented in `docs/pfga-spec.md`.

This spec unifies:

- Frequency Gate behavior
- Pitch Gate Amp behavior
- Null/Noise-Floor validation

And it locks the capture workflow to:

- **Resample print as working truth**
- **Master export as final confirmation only**

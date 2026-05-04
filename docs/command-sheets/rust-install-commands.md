# Rust Install + Build Command Sheet (Windows + Cross-Platform)

## Install / Verify

```powershell
# Install Rust on Windows
.\scripts\setup\install-rust-windows.ps1

# Verify toolchain
rustup --version
rustc --version
cargo --version
```

## Quality Gates

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release --all-targets
```

## One-Command Release Gate

```bash
bash scripts/release/release-gate.sh
```

## Packaging

```bash
bash scripts/release/package-release.sh
```

# Release Process and Feature Integration Gate

This repository uses a **release gate** to enforce build health and accelerate larger feature integration safely.

## Policy

- "Always compiles" is a hard requirement.
- Large business features are merged only after passing the release gate.
- If the trigger word is **release**, run the full gate immediately.

## One-Command Gate

```bash
bash scripts/release/release-gate.sh
```

This command performs, in order:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all-targets --all-features`
4. `cargo build --release --all-targets`
5. `bash scripts/release/package-release.sh`

Generated release archives are written to `dist/`. That directory is intentionally ignored by git because manifests and checksums contain generation-time data.

## Why this process speeds up larger features

- It catches integration breakages at the contract boundaries (`input`, `output`, and shared frame layers) before release packaging.
- It forces packaging validation in the same workflow as compile/test.
- It gives a single operational instruction for engineering and non-engineering stakeholders: run the gate.

## Operational expectation

- Before merge of a large feature branch, run the gate and attach logs in the PR.
- Before producing artifacts, run the gate again on the final merge commit.

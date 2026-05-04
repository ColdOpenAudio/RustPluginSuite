#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

run_step() {
  local label="$1"
  shift
  echo ""
  echo "==> ${label}"
  "$@"
}

run_step "Formatting check" cargo fmt --all -- --check
run_step "Lint (warnings denied)" cargo clippy --all-targets --all-features -- -D warnings
run_step "Tests" cargo test --all-targets --all-features
run_step "Release build" cargo build --release --all-targets
run_step "Release packaging" bash "$ROOT_DIR/scripts/release/package-release.sh"

echo ""
echo "Release gate passed."

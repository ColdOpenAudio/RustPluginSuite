#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
PKG_NAME="rust-plugin-suite"
VERSION="${1:-}"

if [[ -z "$VERSION" ]]; then
  if git -C "$ROOT_DIR" describe --tags --exact-match >/dev/null 2>&1; then
    VERSION="$(git -C "$ROOT_DIR" describe --tags --exact-match)"
  else
    VERSION="v0.1.0"
  fi
fi

VERSION_STRIPPED="${VERSION#v}"
BUNDLE_ROOT="$DIST_DIR/${PKG_NAME}-${VERSION_STRIPPED}"
WINDOWS_SUBDIR="$BUNDLE_ROOT/windows-installer"

rm -rf "$DIST_DIR"
mkdir -p "$WINDOWS_SUBDIR"

cp "$ROOT_DIR/README.md" "$BUNDLE_ROOT/"
cp "$ROOT_DIR/docs/windows-auto-install.md" "$BUNDLE_ROOT/"
cp "$ROOT_DIR/scripts/install-windows-bootstrap.ps1" "$WINDOWS_SUBDIR/"
cp "$ROOT_DIR/scripts/install-windows.ps1" "$WINDOWS_SUBDIR/"
cp "$ROOT_DIR/scripts/install-windows.bat" "$WINDOWS_SUBDIR/"

cat > "$BUNDLE_ROOT/RELEASE-MANIFEST.txt" <<EOF
Package: ${PKG_NAME}
Version: ${VERSION}
Generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
Contents:
- README.md
- windows-auto-install.md
- windows-installer/install-windows-bootstrap.ps1
- windows-installer/install-windows.ps1
- windows-installer/install-windows.bat
EOF

(
  cd "$DIST_DIR"
  zip -r "${PKG_NAME}-${VERSION_STRIPPED}.zip" "${PKG_NAME}-${VERSION_STRIPPED}" >/dev/null
  tar -czf "${PKG_NAME}-${VERSION_STRIPPED}.tar.gz" "${PKG_NAME}-${VERSION_STRIPPED}"
)

(
  cd "$DIST_DIR"
  shasum -a 256 "${PKG_NAME}-${VERSION_STRIPPED}.zip" "${PKG_NAME}-${VERSION_STRIPPED}.tar.gz" > "${PKG_NAME}-${VERSION_STRIPPED}-SHA256SUMS.txt"
)

echo "Release artifacts created in $DIST_DIR"

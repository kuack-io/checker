#!/usr/bin/env bash
#
# Script that builds release artifacts for kuack-checker.
#
# Flag legend:
# -e: exit immediately if one of the commands fails
# -u: throw an error if one of the inputs is not set
# -o pipefail: result is the value of the last command
# +x: do not print all executed commands to terminal
set -euo pipefail
set +x

VERSION=$1
DIST_DIR="dist"
OUTPUT_DIR="release-artifacts"
LINUX_BIN="$DIST_DIR/linux/kuack-checker"
WASM_PKG_DIR="$DIST_DIR/wasm/pkg"

if [[ -z "$VERSION" ]]; then
  echo "[package-assets] Missing release version argument" >&2
  exit 1
fi

if [[ ! -f "$LINUX_BIN" ]]; then
  echo "[package-assets] Expected Linux binary at $LINUX_BIN" >&2
  exit 1
fi

if [[ ! -d "$WASM_PKG_DIR" ]]; then
  echo "[package-assets] Expected WASM pkg directory at $WASM_PKG_DIR" >&2
  exit 1
fi

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

LINUX_ARCHIVE="$OUTPUT_DIR/kuack-checker-${VERSION}-linux-x86_64.tar.gz"
cp "$LINUX_BIN" "$OUTPUT_DIR/kuack-checker"
tar -C "$OUTPUT_DIR" -czf "$LINUX_ARCHIVE" kuack-checker
rm -f "$OUTPUT_DIR/kuack-checker"

tar -C "$DIST_DIR/wasm" -czf "$OUTPUT_DIR/kuack-checker-${VERSION}-wasm32-web.tar.gz" pkg test-browser.html

echo "Created release artifacts in $OUTPUT_DIR:"
ls -lh "$OUTPUT_DIR"

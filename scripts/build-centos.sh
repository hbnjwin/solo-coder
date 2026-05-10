#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "[1/3] Building release binary..."
cargo build --release

echo "[2/3] Stripping binary (if tool exists)..."
if command -v strip >/dev/null 2>&1; then
  strip target/release/data-import-sql || true
fi

echo "[3/3] Preparing dist package..."
rm -rf dist
mkdir -p dist/output
cp target/release/data-import-sql dist/
cp config.example.ini dist/config.ini
cp scripts/run.sh dist/
chmod +x dist/data-import-sql dist/run.sh

GIT_COMMIT="$(git rev-parse --short HEAD 2>/dev/null || echo unknown)"
BUILD_TIME="$(date '+%Y-%m-%d %H:%M:%S %z')"
if command -v sha256sum >/dev/null 2>&1; then
  BIN_SHA256="$(sha256sum dist/data-import-sql | awk '{print $1}')"
else
  BIN_SHA256="sha256sum-not-found"
fi
{
  echo "build_time=$BUILD_TIME"
  echo "git_commit=$GIT_COMMIT"
  echo "binary_sha256=$BIN_SHA256"
} > dist/BUILD_INFO.txt

echo "Build completed."
echo "Files in: $PROJECT_ROOT/dist"
echo "Build info:"
echo "  $PROJECT_ROOT/dist/BUILD_INFO.txt"
echo "Run:"
echo "  cd dist"
echo "  ./run.sh"

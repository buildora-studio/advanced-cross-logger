#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

# Targets supported for cross-compilation.
# Each entry: "<napi-target>|<rust-target>"
ALL_TARGETS=(
  "darwin-arm64|aarch64-apple-darwin"
  "darwin-x64|x86_64-apple-darwin"
  "linux-x64-gnu|x86_64-unknown-linux-gnu"
  "linux-arm64-gnu|aarch64-unknown-linux-gnu"
  "win32-x64-msvc|x86_64-pc-windows-msvc"
  "win32-arm64-msvc|aarch64-pc-windows-msvc"
)

cd "$REPO_ROOT/bindings/node"
npm install

if [[ "${1:-}" == "--all-platforms" ]]; then
  echo "Building for all platforms..."
  for entry in "${ALL_TARGETS[@]}"; do
    napi_target="${entry%%|*}"
    rust_target="${entry##*|}"

    echo "→ $napi_target ($rust_target)"

    # Install the Rust target if missing
    rustup target add "$rust_target" 2>/dev/null || true

    npx napi build --platform --release --target "$rust_target" || {
      echo "  skipped (cross-compilation toolchain not available for $rust_target)"
    }
  done
else
  # Current platform only (development)
  npm run build
fi

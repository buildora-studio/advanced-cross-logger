#!/usr/bin/env bash
set -euo pipefail

SCRIPTS_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPTS_DIR/../.." && pwd)"

# Build all Rust crates first
cd "$REPO_ROOT"
cargo build --release

"$SCRIPTS_DIR/python.sh"
"$SCRIPTS_DIR/node.sh"
"$SCRIPTS_DIR/go.sh"
"$SCRIPTS_DIR/java.sh"

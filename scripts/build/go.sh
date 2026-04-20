#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
LIBDIR="$REPO_ROOT/target/release"
PC_DIR="$REPO_ROOT/bindings/go"
PC_FILE="$PC_DIR/cross_logger.pc"

# Build the Rust static library
cd "$REPO_ROOT"
cargo build --release -p cross_logger_go

# Generate cross_logger.pc from the template, pointing at the local build
sed "s|@LIBDIR@|$LIBDIR|g" "$PC_DIR/cross_logger.pc.in" > "$PC_FILE"

# Clear Go's CGo cache so it picks up changes in the Rust static lib
go clean -cache

# Build the Go package with the local .pc file on the search path
cd "$PC_DIR"
PKG_CONFIG_PATH="$PC_DIR" go build ./...

echo "PKG_CONFIG_PATH=$PC_DIR go build ./..."

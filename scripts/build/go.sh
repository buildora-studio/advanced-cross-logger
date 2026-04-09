#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

cd "$REPO_ROOT"
cargo build --release -p cross_logger_go

cd "$REPO_ROOT/bindings/go"
go build ./...

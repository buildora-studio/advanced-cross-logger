#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

cd "$REPO_ROOT"
cargo build --release -p cross_logger_java

cd "$REPO_ROOT/bindings/java"
mvn compile

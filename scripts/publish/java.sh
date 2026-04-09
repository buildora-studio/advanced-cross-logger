#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

# Build the native library
cd "$REPO_ROOT"
cargo build --release -p cross_logger_java

# Bundle the native library into the JAR resources
RESOURCES_DIR="$REPO_ROOT/bindings/java/src/main/resources"
mkdir -p "$RESOURCES_DIR"

for lib in \
    "$REPO_ROOT/target/release/libcross_logger.dylib" \
    "$REPO_ROOT/target/release/libcross_logger.so"; do
    [[ -f "$lib" ]] && cp "$lib" "$RESOURCES_DIR/"
done

# Deploy to Maven Central (requires GPG + OSSRH credentials in ~/.m2/settings.xml)
cd "$REPO_ROOT/bindings/java"
mvn clean deploy -P release

#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

cd "$REPO_ROOT/bindings/node"

# Generate per-platform package directories and update optionalDependencies
npx napi prepublish --skip-gh-release

# Publish the per-platform packages first
for dir in npm/*/; do
  [[ -d "$dir" ]] || continue
  echo "Publishing $dir..."
  npm publish "$dir" --access public
done

# Publish the main package
npm publish --access public

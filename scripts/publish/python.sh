#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

if [[ -z "${MATURIN_PYPI_TOKEN:-}" ]]; then
    echo "error: MATURIN_PYPI_TOKEN is not set" >&2
    exit 1
fi

cd "$REPO_ROOT/bindings/python"
maturin publish

#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

usage() {
    echo "Usage: $0 [node|python|go|java|all]"
    echo ""
    echo "  node    Run the Node.js example"
    echo "  python  Run the Python example"
    echo "  go      Run the Go example"
    echo "  java    Run the Java example"
    echo "  all     Run all examples"
    exit 1
}

run_node() {
    echo "=== Node.js ==="
    bash "$REPO_ROOT/scripts/build/node.sh"
    cd "$REPO_ROOT/examples/node"
    npm start
}

run_python() {
    echo "=== Python ==="
    cd "$REPO_ROOT/examples/python"

    if [[ ! -d ".venv" ]]; then
        echo "Creating virtualenv..."
        python3 -m venv .venv
    fi

    source "$REPO_ROOT/examples/python/.venv/bin/activate"
    pip install --quiet maturin
    cd "$REPO_ROOT/bindings/python"
    maturin develop
    cd "$REPO_ROOT/examples/python"
    python main.py
}

run_go() {
    echo "=== Go ==="

    bash "$REPO_ROOT/scripts/build/go.sh"

    cd "$REPO_ROOT/examples/go"
    PKG_CONFIG_PATH="$REPO_ROOT/bindings/go" go run main.go
}

run_java() {
    echo "=== Java ==="
    cd "$REPO_ROOT"
    cargo build --release -p cross_logger_java 2>/dev/null
    cd "$REPO_ROOT/bindings/java"
    mvn -q install
    cd "$REPO_ROOT/examples/java"
    mvn -q compile exec:exec
}

TARGET="${1:-}"

case "$TARGET" in
    node)   run_node   ;;
    python) run_python ;;
    go)     run_go     ;;
    java)   run_java   ;;
    all)
        run_node
        echo ""
        run_python
        echo ""
        run_go
        echo ""
        run_java
        ;;
    *) usage ;;
esac

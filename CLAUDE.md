# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

`cross-logger` is a Rust logging library designed to be exposed to multiple language runtimes via FFI/bindings. The core logic lives in one place and is wrapped for each target language.

## Build Commands

```bash
# Build all workspace crates
cargo build

# Build a specific crate
cargo build -p core
cargo build -p cross_logger_python

# Run tests across workspace
cargo test

# Run tests for a single crate
cargo test -p core

# Run a single test by name
cargo test -p core test_name
```

## Architecture

The workspace is split into two layers:

1. **`core/`** — Pure Rust library crate. Contains `LogLevel` enum and `print_log(level, message)`. No external dependencies. All new logging logic goes here.

2. **`bindings/`** — One sub-crate per target language, each depending on `core`:
   - `bindings/python/` — `pyo3`-based extension module (`cdylib`), compiled to a `.so`/`.pyd` loadable by Python
   - `bindings/node/`, `bindings/go/rust-go/`, `bindings/java/rust-jni/` — planned, directories/workspace entries exist but contain no source yet

Each binding crate re-exports `core` types/functions through its language's FFI mechanism. New language targets should follow the same pattern: add a `Cargo.toml` with `crate-type = ["cdylib"]`, depend on `core`, and register it in the root `Cargo.toml` workspace members.

## Python Binding Notes

The Python binding uses `pyo3 0.20`. To build a usable `.so`:

```bash
# Requires maturin
pip install maturin
cd bindings/python
maturin develop   # installs into current Python env for dev
maturin build     # produces a wheel
```

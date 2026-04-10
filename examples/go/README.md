# Go example

Demonstrates the basic usage of `cross-logger` from Go.

## Setup

1. Build the Rust static library and generate the local `pkg-config` file:

```bash
bash scripts/build/go.sh
```

2. Run the example with `PKG_CONFIG_PATH` pointing at the local `.pc` file:

```bash
cd examples/go
PKG_CONFIG_PATH=../../bindings/go go run main.go
```

Expected output:

```
[INFO] Server started on port 3000
[WARN] Memory usage above 80%
[ERROR] Failed to connect to database
```

## How the module is resolved

`go.mod` uses a `replace` directive to point `cross-logger` at the local binding:

```
replace cross-logger => ../../bindings/go
```

This lets the example import the binding as if it were a published module, without needing to publish it first.

## Using the published module

Once `cross-logger` is tagged and available via the Go module proxy, remove the `replace` directive and run:

```bash
go get cross-logger@v0.1.0
```

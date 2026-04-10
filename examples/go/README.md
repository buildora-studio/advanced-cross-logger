# Go example

Demonstrates the basic usage of `cross-logger` from Go.

## Setup

1. Build the Rust static library:

```bash
cargo build --release -p cross_logger_go
```

2. Run the example:

```bash
cd examples/go
go run main.go
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

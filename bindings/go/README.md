# cross-logger — Go binding

Go binding for cross-logger, built with CGo. The Rust core is compiled to a static library and linked into the Go binary at build time.

## How it works

```
core/src/lib.rs              Rust logging logic (LogLevel + print_log)
      ↓  (dependency)
rust-go/src/lib.rs           C FFI functions — unsafe extern "C" exported symbols
      ↓  (cargo build)
target/release/
  libcross_logger.a          Static library linked by CGo at build time
  libcross_logger.dylib/.so  Dynamic library (also produced, not used by Go)
      ↓
logger.go                    CGo wrapper — declares C signatures and exposes Go functions
```

### Rust → Go (CGo)

`rust-go/src/lib.rs` exports plain C functions using `unsafe extern "C"` and `#[unsafe(no_mangle)]`. Each function receives a `*const c_char`, converts it to a Rust `&str`, and delegates to `core`.

`logger.go` declares those C signatures in the CGo preamble and wraps them in idiomatic Go functions. Go strings are converted to C strings with `C.CString` and freed with `defer C.free`:

```go
/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lcross_logger -ldl -lpthread
#include <stdlib.h>
void log_info(const char* message);
*/
import "C"
import "unsafe"

func LogInfo(message string) {
    cs := C.CString(message)
    defer C.free(unsafe.Pointer(cs))
    C.log_info(cs)
}
```

### Library path

The `LDFLAGS` directive uses `${SRCDIR}` — CGo replaces this with the absolute path of the directory containing `logger.go` (`bindings/go/`). The static library lives two levels up in the Cargo workspace output directory:

```
${SRCDIR}/../../target/release  →  <repo>/target/release
```

The Rust static library must be built before any `go build` or `go run`.

## Building from source

```bash
# 1. Build the Rust static library
cargo build --release -p cross_logger_go

# 2. Build the Go package
cd bindings/go
go build ./...
```

## API

```go
import crosslogger "cross-logger"

crosslogger.LogInfo("message")   // [INFO] message
crosslogger.LogWarn("message")   // [WARN] message
crosslogger.LogError("message")  // [ERROR] message
```

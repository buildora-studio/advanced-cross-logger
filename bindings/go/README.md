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
#cgo pkg-config: cross_logger
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

### Library resolution via pkg-config

`logger.go` uses `#cgo pkg-config: cross_logger` instead of a hardcoded path. CGo calls `pkg-config --libs cross_logger` at build time to get the linker flags, which reads from a `cross_logger.pc` file on `PKG_CONFIG_PATH`.

- **Production**: install `cross_logger.pc` alongside the library into a standard system prefix (e.g. `/usr/local/lib/pkgconfig/`).
- **Development**: `scripts/build/go.sh` generates `cross_logger.pc` from `cross_logger.pc.in` with the local `target/release` path and sets `PKG_CONFIG_PATH` automatically.

`cross_logger.pc` is generated — it is not committed to the repository.

## Building from source

```bash
bash scripts/build/go.sh
```

This builds the Rust static library, generates `cross_logger.pc`, and runs `go build ./...` with `PKG_CONFIG_PATH` set to the local binding directory.

## API

```go
import crosslogger "cross-logger"

crosslogger.LogInfo("message")   // [INFO] message
crosslogger.LogWarn("message")   // [WARN] message
crosslogger.LogError("message")  // [ERROR] message
```

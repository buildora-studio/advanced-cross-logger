# cross-logger — Python binding

Python binding for cross-logger, built with [pyo3](https://pyo3.rs). The Rust core is compiled to a native `.so` extension and wrapped in a Python package.

## How it works

```
core/src/lib.rs              Rust logging logic (LogLevel + print_log)
      ↓  (dependency)
src/lib.rs                   pyo3 functions — wraps each Rust fn as a Python callable
      ↓  (maturin build)
cross_logger/
  cross_logger.abi3.so       Native extension loaded at import time
  __init__.py                Re-exports the native functions into the package namespace
```

### Rust → Python (pyo3)

Each function in `src/lib.rs` is annotated with `#[pyfunction]` and registered in the module with `wrap_pyfunction!`. When maturin builds the project, it compiles the Rust code into a `.so` extension that Python can import natively.

```rust
#[pyfunction]
fn log_info(message: &str) {
    print_log(LogLevel::Info, message);
}

#[pymodule]
fn cross_logger(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(log_info, m)?)?;
    // ...
}
```

### Stable ABI (`abi3-py39`)

The binding is compiled with pyo3's `abi3-py39` feature. This targets Python's [Stable ABI](https://docs.python.org/3/c-api/stable.html), which produces a single wheel (`cross_logger-*.abi3.so`) compatible with Python 3.9 and any later version — including versions newer than pyo3's own support ceiling.

### Package layout

`cross_logger/` is a standard Python package. Its `__init__.py` imports from the native extension and re-exports the public API:

```python
from .cross_logger import log_info, log_warn, log_error
```

This lets consumers write `import cross_logger` and call `cross_logger.log_info(...)` directly, without knowing about the underlying `.so`.

## Building from source

Requires [maturin](https://www.maturin.rs):

```bash
pip install maturin
```

Install into a virtualenv for development:

```bash
python3 -m venv .venv && source .venv/bin/activate
maturin develop
```

Build a distributable wheel:

```bash
maturin build --release
# wheel written to ../../target/wheels/
```

## API

```python
import cross_logger

cross_logger.log_info("message")   # [INFO] message
cross_logger.log_warn("message")   # [WARN] message
cross_logger.log_error("message")  # [ERROR] message
```

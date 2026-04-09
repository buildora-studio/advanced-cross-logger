# Maintainer guide

## Repository layout

```
core/                        # Rust library — single source of truth for all log logic
bindings/
  python/                    # pyo3 extension module (cdylib)
  node/                      # napi-rs Node.js addon (cdylib)
  go/
    rust-go/                 # Rust cdylib + staticlib exposing C FFI
    logger.go                # CGo wrapper (links against rust-go)
    go.mod
  java/
    rust-jni/                # Rust JNI implementation (cdylib)
    src/main/java/           # Java class with native methods
    pom.xml
Cargo.toml                   # Workspace root (resolver = "3")
```

All business logic lives in `core/src/lib.rs`. Bindings only translate types and delegate to `core`.

## Development requirements

| Tool | Min version |
|------|-------------|
| Rust / Cargo | 1.94 |
| Python | 3.9 |
| maturin | latest |
| Node.js / npm | 18 |
| Go | 1.21 |
| Java (JDK) | 11 |
| Maven | 3.x |

## Checking and linting

```bash
# Check all crates compile
cargo check

# Lint all crates
cargo clippy
```

Run clippy before every PR. The CI treats warnings as errors on the Rust side.

## Building

```bash
scripts/build/all.sh        # all bindings
scripts/build/python.sh
scripts/build/node.sh
scripts/build/go.sh
scripts/build/java.sh
```

Or directly with Cargo when iterating on the Rust side:

```bash
cargo build --release -p cross_logger_python
cargo build --release -p cross_logger_node
cargo build --release -p cross_logger_go
cargo build --release -p cross_logger_java
```

## Testing

```bash
cargo test
```

## Adding a new language binding

1. Create `bindings/<lang>/` with a `Cargo.toml` (crate-type `["cdylib"]`).
2. Add the new crate to the workspace `members` in the root `Cargo.toml`.
3. Depend on `core` using the package alias to avoid shadowing the Rust built-in:
   ```toml
   cross_logger_core = { package = "core", path = "../../core" }
   ```
4. Import as `cross_logger_core` in `src/lib.rs` (never `use core::...`).
5. Implement the language-specific FFI layer delegating to `cross_logger_core::print_log`.
6. Verify with `cargo clippy -p <crate_name>` before opening a PR.

## Publishing

### PyPI (Python)

`maturin` compiles the Rust extension and builds wheels for each platform. Requires a PyPI API token in `MATURIN_PYPI_TOKEN` (or configured in `~/.pypirc`).

```bash
scripts/publish/python.sh
```

The `abi3-py39` feature produces a single wheel compatible with Python 3.9+. For multi-platform releases, run this script in CI on each target OS/arch and upload all wheels in a single PyPI release.

---

### npm (Node.js)

Requires being logged in to npm (`npm login`). The `@cross-logger` scope must exist on npm before the first publish.

```bash
scripts/publish/node.sh
```

For multi-platform releases, build on each target OS/arch and publish platform-specific optional packages (the standard napi-rs multi-platform pattern). See the `@napi-rs/cli` docs for `napi prepublish`.

---

### Maven Central (Java)

The script compiles the Rust native library, copies it into the JAR resources, and deploys to Maven Central via OSSRH. Requires GPG signing and OSSRH credentials in `~/.m2/settings.xml`, and the `pom.xml` must include `maven-gpg-plugin`, `maven-source-plugin`, and `maven-javadoc-plugin`.

```bash
scripts/publish/java.sh
```

Before deploying, update `CrossLogger.java` to load the native library from the classpath instead of `System.loadLibrary` so consumers don't need to manage `java.library.path` themselves.

---

### Go module proxy

Go modules are distributed via source — tag the release and the Go module proxy picks it up automatically.

```bash
scripts/publish/go.sh 0.1.0
```

The tag prefix `go/` scopes it to the Go module at `bindings/go`. Before the first release, update the module path in `bindings/go/go.mod` to match the repository URL (e.g. `github.com/your-org/cross-logger/bindings/go`).

---

## Dependency notes

### Python (`pyo3 0.23`)
- Uses `abi3-py39` feature for forward compatibility with Python versions newer than pyo3's supported maximum.
- Build with `maturin`, not raw `cargo build` — the linker flags for `.so` extension modules require it.
- Module init signature must use `Bound<'_, PyModule>` (pyo3 0.22+ API).

### Node.js (`napi 2`)
- Requires `napi-build` as a build dependency and a `build.rs` calling `napi_build::setup()`.
- Build with `npm run build` inside `bindings/node/` (delegates to `@napi-rs/cli`).

### Go (C FFI)
- Rust side exposes `pub unsafe extern "C"` functions — must be marked `unsafe` and include a `# Safety` doc comment.
- Go side uses CGo; the `LDFLAGS` in `logger.go` point to `rust-go/target/release`.
- Build the Rust static lib before running `go build`.

### Java (JNI)
- JNI function names follow `Java_<package_underscored>_<Class>_<method>` — any rename of the Java class or package must be reflected in the Rust function names.
- The `.dylib`/`.so` path must be passed via `-Djava.library.path` at runtime.

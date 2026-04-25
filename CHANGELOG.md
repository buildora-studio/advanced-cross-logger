# Changelog

## [Unreleased]

### Changed

- **`log()` now requires a UUID `id` parameter (breaking change — all bindings)**
  - `LoggerConfig::log(level, message)` → `LoggerConfig::log(level, id, message)` across
    core, Node, Go, Java, and Python.
  - The `id` identifies the log entry and appears in every output line. Bindings accept it
    as a string and parse it internally; invalid UUIDs fall back to the nil UUID.

- **Log output format simplified to `LEVEL [name] [uuid] payload`**
  - Replaced the JSON envelope (`{ timestamp, severity, logger, message }`) with a
    plain single-line format: `INFO [my-app] [<uuid>] message text`.
  - JSON message payloads are inlined as-is; string payloads are printed verbatim.
  - Removed `chrono` from `core` — timestamps are no longer emitted by the library.
  - Terminal colorization now only highlights the level label instead of the full
    pretty-printed JSON block (`colorize_terminal_line` replaces `colorize_terminal`).

- **`uuid` crate added to `core` dependencies**
  - `core/Cargo.toml` now depends on `uuid = { version = "1", features = ["v4"] }`.
  - `Uuid` is re-exported from `cross_logger_core` so bindings can use it directly.

### Fixed

- **Python binding — build failure (linker undefined symbols)**
  - `cargo build` desde la raíz del workspace intentaba enlazar el binding de Python
    directamente, pero `pyo3` requiere que los símbolos de CPython se resuelvan en
    tiempo de ejecución (cuando Python carga el `.so`), no en tiempo de enlace.
  - Se añadió `default-members` en `Cargo.toml` del workspace para excluir
    `bindings/python` de las builds por defecto. El crate sigue en `members` y se
    puede compilar explícitamente con `maturin develop` / `maturin build`.

- **Go binding — símbolos `log_info` / `log_warn` / `log_error` no encontrados**
  - Causa raíz: tanto el binding de Python como el de Go usaban `name = "cross_logger"`
    en su sección `[lib]`, por lo que ambos producían `libcross_logger.dylib` en
    `target/release/`. El enlazador de macOS prefiere `.dylib` sobre `.a`, y la
    dylib presente era la del binding de Python (`_PyInit_cross_logger`), que no
    exporta las funciones FFI de Go.
  - Se eliminó `cdylib` de `crate-type` en `bindings/go/rust-go/Cargo.toml`
    (solo se mantiene `staticlib`). CGo únicamente necesita una librería estática.
  - Se actualizó `bindings/go/cross_logger.pc` (y `.pc.in`) para enlazar mediante
    la ruta absoluta al archivo `libcross_logger.a` en lugar de `-lcross_logger`,
    evitando que el enlazador pueda seleccionar una dylib con el mismo nombre.

- **Go binding — `cross_logger.pc` con ruta hardcodeada**
  - El archivo `cross_logger.pc` podía contener la ruta absoluta de otra máquina si
    se copiaba o clonaba el repo en un entorno diferente.
  - `scripts/run-example.sh` ahora siempre regenera el `.pc` desde el template
    `cross_logger.pc.in` antes de correr el ejemplo, eliminando la condición
    `if [[ ! -f ... ]]` que omitía la regeneración cuando el archivo ya existía.

- **Go binding — caché de CGo no detectaba cambios en la librería Rust**
  - Cuando se modificaba `core`, Go servía el binario cacheado ignorando el nuevo
    `libcross_logger.a`. `scripts/build/go.sh` ahora ejecuta `go clean -cache` antes
    de compilar para garantizar que el cambio se propague.

- **Node y Python — ejemplos no recompilaban al cambiar `core`**
  - `scripts/run-example.sh` no reconstruía los bindings de Node ni Python antes de
    correr sus ejemplos, por lo que mostraban binarios desactualizados.
  - `run_node` ahora llama a `scripts/build/node.sh` antes de `npm start`.
  - `run_python` ahora siempre ejecuta `maturin develop` (antes solo lo hacía si el
    `.venv` no existía).

- **Ejemplos Python, Go y Java — firma de `log()` desactualizada**
  - Los tres ejemplos seguían llamando a `log(level, message)` tras el breaking change
    que agregó el parámetro `id`.
  - `examples/python/main.py`: agrega `import uuid` y pasa `str(uuid.uuid4())` como `id`.
  - `examples/go/main.go`: agrega dependencia `github.com/google/uuid` y pasa
    `uuid.New().String()` como `id`.
  - `examples/java/Main.java`: usa `UUID.randomUUID().toString()` y pasa `id` en cada
    llamada a `.log()`.

- **`run-example.sh` — `maturin` no encontrado en builds limpias**
  - El script activaba el venv pero asumía que `maturin` ya estaba instalado globalmente.
  - `run_python` ahora ejecuta `pip install --quiet maturin` dentro del venv antes de
    llamar a `maturin develop`, haciendo el setup autocontenido.

- **Go example — dependencia `pkg-config` faltante**
  - La build de Go requiere `pkg-config` para resolver `cross_logger.pc`; en macOS
    se instala con `brew install pkg-config`.

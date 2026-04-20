# Changelog

## [Unreleased]

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

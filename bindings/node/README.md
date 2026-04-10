# @cross-logger/node

Node.js binding for cross-logger, built with [napi-rs](https://napi.rs). The Rust core is compiled to a native `.node` addon and exposed to JavaScript and TypeScript.

## How it works

```
core/src/lib.rs          Rust logging logic (LogLevel + print_log)
      ↓  (dependency)
src/lib.rs               napi-rs functions — translates JS strings to Rust,
                         delegates to core
      ↓  (napi build)
index.<platform>.node    Native addon loaded at runtime by Node.js
      ↓
index.js                 CJS loader — resolves the right binary per platform
index.mjs                ESM wrapper — re-exports named functions for ESM consumers
index.d.ts               TypeScript declarations
```

### Rust → Node.js (napi-rs)

Each function in `src/lib.rs` is annotated with `#[napi]`. When `napi build` runs, it compiles the Rust code into a platform-specific `.node` binary (e.g. `index.darwin-arm64.node`) that Node.js can load natively via `require()`.

### Platform resolution (`index.js`)

The CJS loader resolves the native binary at runtime in two steps:

1. **Published package** — tries the platform-specific optional package installed by npm (e.g. `@cross-logger/node-darwin-arm64`). These are declared as `optionalDependencies` so npm installs only the one matching the consumer's OS and architecture.
2. **Local fallback** — if no package is found, tries a `index.<platform>-<arch>.node` file in the same directory. This is the path used when building from source.

### ESM / CJS interop (`index.mjs`)

The native addon is always a CJS module. When `module.exports` is assigned a dynamic value (as our loader does), Node.js cannot statically detect named exports, so ESM consumers get an error when using named imports.

`index.mjs` solves this by explicitly re-exporting each function:

```js
// index.mjs
const native = require('./index.js')
export const logInfo    = native.logInfo
export const logWarning = native.logWarning
export const logError   = native.logError
```

The `exports` field in `package.json` routes consumers to the right file:

```json
"exports": {
  ".": {
    "types":   "./index.d.ts",
    "import":  "./index.mjs",
    "require": "./index.js"
  }
}
```

## Building from source

```bash
npm install
npm run build        # release
npm run build:debug  # debug
```

For all platforms at once (requires the Rust cross-compilation toolchains):

```bash
../../scripts/build/node.sh --all-platforms
```

## Supported platforms

| Platform        | Package                               |
|-----------------|---------------------------------------|
| macOS arm64     | `@cross-logger/node-darwin-arm64`     |
| macOS x64       | `@cross-logger/node-darwin-x64`       |
| Linux x64       | `@cross-logger/node-linux-x64-gnu`    |
| Linux arm64     | `@cross-logger/node-linux-arm64-gnu`  |
| Windows x64     | `@cross-logger/node-win32-x64-msvc`   |
| Windows arm64   | `@cross-logger/node-win32-arm64-msvc` |

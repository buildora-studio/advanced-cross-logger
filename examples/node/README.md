# Node.js example

Demonstrates the basic usage of `@cross-logger/node` from TypeScript.

## Setup

1. Install dependencies:

```bash
npm install
```

2. Link the binding from source:

```bash
cd ../../bindings/node
npm run build
npm link
cd -
npm link @cross-logger/node
```

3. Run the example:

```bash
npm start
```

Expected output:

```
[INFO] Server started on port 3000
[WARN] Memory usage above 80%
[ERROR] Failed to connect to database
```

## Using the published package

If `@cross-logger/node` is already published on npm, skip the build and link steps:

```bash
npm install
npm start
```

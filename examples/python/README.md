# Python example

Demonstrates the basic usage of `cross_logger` from Python.

## Setup

1. Create and activate a virtualenv:

```bash
python3 -m venv .venv
source .venv/bin/activate        # macOS / Linux
.venv\Scripts\activate           # Windows
```

2. Build and install the binding from source:

```bash
cd ../../bindings/python
maturin develop
cd -
```

3. Run the example:

```bash
python main.py
```

Expected output:

```
[INFO] Server started on port 3000
[WARN] Memory usage above 80%
[ERROR] Failed to connect to database
```

## Using the published package

If `cross-logger` is already published on PyPI, skip the maturin step and install directly:

```bash
pip install cross-logger
python main.py
```

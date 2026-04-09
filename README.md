# cross-logger

A logging library available for Python, Node.js, Go, and Java. The core is written in Rust and exposed to each language via native bindings.

## Python

**Requirements:** Python 3.9+

```bash
pip install cross-logger
```

```python
import cross_logger

cross_logger.log_info("hello")
cross_logger.log_warn("watch out")
cross_logger.log_error("something broke")
```

---

## Node.js

**Requirements:** Node.js 18+

```bash
npm install @cross-logger/node
```

```ts
import { logInfo, logWarning, logError } from '@cross-logger/node'

logInfo('hello')
logWarning('watch out')
logError('something broke')
```

---

## Go

**Requirements:** Rust 1.94+, Go 1.21+, CGo enabled.

The Go binding uses CGo and links against the Rust static library, so it must be compiled from source.

1. Build the Rust static library:

```bash
cargo build --release -p cross_logger_go
```

2. Build the Go package:

```bash
cd bindings/go
go build ./...
```

Usage:

```go
import crosslogger "cross-logger"

crosslogger.LogInfo("hello")
crosslogger.LogWarn("watch out")
crosslogger.LogError("something broke")
```

---

## Java

**Requirements:** Rust 1.94+, Java 11+, Maven 3.x.

The Java binding uses JNI and requires the native library to be compiled and available at runtime.

1. Build the native library:

```bash
cargo build --release -p cross_logger_java
# output: target/release/libcross_logger.dylib  (macOS)
#         target/release/libcross_logger.so      (Linux)
```

2. Compile the Java sources:

```bash
cd bindings/java
mvn compile
```

3. Run pointing to the native library:

```bash
java -Djava.library.path=../../target/release \
     -cp target/classes \
     com.crosslogger.CrossLogger
```

Usage:

```java
import com.crosslogger.CrossLogger;

CrossLogger.logInfo("hello");
CrossLogger.logWarn("watch out");
CrossLogger.logError("something broke");
```


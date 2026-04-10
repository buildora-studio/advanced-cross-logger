# cross-logger — Java binding

Java binding for cross-logger, built with [jni-rs](https://github.com/jni-rs/jni-rs). The Rust core is compiled to a native shared library and called from Java via JNI.

## How it works

```
core/src/lib.rs              Rust logging logic (LogLevel + print_log)
      ↓  (dependency)
rust-jni/src/lib.rs          JNI functions — bridges Java calls to Rust
      ↓  (cargo build)
libcross_logger.dylib/.so    Native shared library loaded at runtime
      ↓
CrossLogger.java             Java class with native method declarations
```

### Rust → Java (JNI)

Each function in `rust-jni/src/lib.rs` is exported with `extern "system"` and a name that follows the JNI convention: `Java_<package>_<Class>_<method>`.

```rust
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_crosslogger_CrossLogger_logInfo(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
) {
    let msg: String = env.get_string(&message).unwrap().into();
    print_log(LogLevel::Info, &msg);
}
```

When the Java class is loaded, the JVM resolves these symbols from the native library and links them to the `native` methods declared in `CrossLogger.java`.

> Any rename of the Java package or class must be reflected in the Rust function names.

### Native library loading

`CrossLogger.java` loads the library in a static block:

```java
static {
    System.loadLibrary("cross_logger");
}
```

The JVM searches for `libcross_logger.dylib` (macOS) or `libcross_logger.so` (Linux) in the paths listed in `java.library.path`. This must be set at JVM startup via `-Djava.library.path=<path/to/target/release>`.

## Building from source

Build the native library:

```bash
cargo build --release -p cross_logger_java
```

Install the JAR into the local Maven repository:

```bash
cd bindings/java
mvn install
```

## API

```java
import com.crosslogger.CrossLogger;

CrossLogger.logInfo("message");   // [INFO] message
CrossLogger.logWarn("message");   // [WARN] message
CrossLogger.logError("message");  // [ERROR] message
```

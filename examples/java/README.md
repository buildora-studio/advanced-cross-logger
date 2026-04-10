# Java example

Demonstrates the basic usage of `cross-logger` from Java.

## Setup

1. Build the native library:

```bash
cargo build --release -p cross_logger_java
```

2. Install the binding JAR into the local Maven repository:

```bash
cd ../../bindings/java
mvn install
cd -
```

3. Compile and run the example:

```bash
mvn compile exec:exec
```

Expected output:

```
[INFO] Server started on port 3000
[WARN] Memory usage above 80%
[ERROR] Failed to connect to database
```

## How the native library is resolved

The `pom.xml` passes `-Djava.library.path=../../target/release` to the forked JVM via `exec-maven-plugin`. This is where `libcross_logger.dylib` (macOS) or `libcross_logger.so` (Linux) lives after `cargo build --release`.

## Using the published package

If `com.crosslogger:cross-logger` is already published on Maven Central, skip steps 1 and 2 and add the dependency to your `pom.xml`:

```xml
<dependency>
    <groupId>com.crosslogger</groupId>
    <artifactId>cross-logger</artifactId>
    <version>0.1.0</version>
</dependency>
```

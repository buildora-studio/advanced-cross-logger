package com.crosslogger;

/**
 * Named logger instance. Create one per service or component.
 *
 * <p>Holds a native pointer to a Rust {@code LoggerConfig}. Must be closed
 * when no longer needed to release native memory. Implements
 * {@link AutoCloseable} for use in try-with-resources.
 *
 * <pre>{@code
 * try (LoggerConfig logger = new LoggerConfig("payments", LogLevel.INFO)) {
 *     logger.log(LogLevel.INFO, "service started");
 * }
 * }</pre>
 */
public class LoggerConfig implements AutoCloseable {

    static {
        System.loadLibrary("cross_logger_jni");
    }

    private final long nativeHandle;

    /**
     * Creates a logger with name and minimum level.
     *
     * @param name     identifier shown in every log entry
     * @param minLevel entries below this level are dropped; use {@link LogLevel} constants
     */
    public LoggerConfig(String name, int minLevel) {
        this(name, minLevel, false, null, 0);
    }

    /**
     * Creates a logger with cloud mode.
     *
     * @param name     identifier shown in every log entry
     * @param minLevel entries below this level are dropped
     * @param isCloud  cloud-compatible output format
     */
    public LoggerConfig(String name, int minLevel, boolean isCloud) {
        this(name, minLevel, isCloud, null, 0);
    }

    /**
     * Creates a logger with all options.
     *
     * @param name            identifier shown in every log entry
     * @param minLevel        entries below this level are dropped
     * @param isCloud         cloud-compatible output format
     * @param obfuscateKeys   keys whose values are replaced with "***" in JSON messages
     * @param obfuscateDepth  max nesting depth to search for obfuscateKeys; 0 disables
     */
    public LoggerConfig(String name, int minLevel, boolean isCloud, String[] obfuscateKeys, int obfuscateDepth) {
        this.nativeHandle = nativeCreate(name, minLevel, isCloud, obfuscateKeys, obfuscateDepth);
    }

    /**
     * Emits a log entry if {@code level >= minLevel}.
     * If {@code message} is a JSON string it is embedded as a nested object.
     *
     * @param level   one of the {@link LogLevel} constants
     * @param message plain string or JSON string
     */
    public void log(int level, String message) {
        nativeLog(nativeHandle, level, message);
    }

    /** Releases native memory. Safe to call multiple times. */
    @Override
    public void close() {
        nativeDestroy(nativeHandle);
    }

    private native long nativeCreate(String name, int minLevel, boolean isCloud, String[] obfuscateKeys, int obfuscateDepth);
    private native void nativeLog(long handle, int level, String message);
    private native void nativeDestroy(long handle);
}

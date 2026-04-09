package com.crosslogger;

public class CrossLogger {
    private CrossLogger() {
        // Utility class, should not be instantiated
    }

    static {
        System.loadLibrary("cross_logger");
    }

    public static native void logInfo(String message);
    public static native void logWarn(String message);
    public static native void logError(String message);
}

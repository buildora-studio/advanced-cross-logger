package com.crosslogger;

/** Severity levels for log entries. Pass these constants to LoggerConfig.log(). */
public final class LogLevel {
    public static final int OFF   = -1;
    public static final int SILLY =  0;
    public static final int DEBUG =  1;
    public static final int INFO  =  2;
    public static final int WARN  =  3;
    public static final int ERROR =  4;
    public static final int FATAL =  5;

    private LogLevel() {}
}

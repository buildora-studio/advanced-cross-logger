import com.crosslogger.LoggerConfig;
import com.crosslogger.LogLevel;

public class Main {
    // Singleton pattern — cada desarrollador implementa el suyo
    private static LoggerConfig instance;

    private static synchronized LoggerConfig getLogger() {
        if (instance == null) {
            instance = new LoggerConfig("global", LogLevel.INFO,
                false,
                new String[]{"password", "token"},
                2
            );
        }
        return instance;
    }

    public static void main(String[] args) {

        // --- Uso básico (solo campos requeridos) ---

        try (LoggerConfig logger = new LoggerConfig("my-app", LogLevel.INFO)) {
            logger.log(LogLevel.SILLY, "starting trace");
            logger.log(LogLevel.DEBUG, "config loaded");
            logger.log(LogLevel.INFO,  "server ready on :8080");
            logger.log(LogLevel.WARN,  "memory usage above 80%");
            logger.log(LogLevel.ERROR, "failed to connect to db");
            logger.log(LogLevel.FATAL, "unrecoverable panic");
        }

        // --- Con isCloud ---

        try (LoggerConfig logger = new LoggerConfig("payments", LogLevel.INFO, true)) {
            logger.log(LogLevel.INFO, "service started");
        }

        // --- Con obfuscation ---

        try (LoggerConfig logger = new LoggerConfig(
            "auth", LogLevel.DEBUG, false,
            new String[]{"password", "token", "ssn"}, 2
        )) {
            logger.log(LogLevel.INFO, "{\"user\":\"alice\",\"password\":\"s3cr3t\"}");
            logger.log(LogLevel.WARN, "{\"session\":{\"token\":\"abc123\",\"expires\":3600}}");
        }

        // --- Con todo ---

        try (LoggerConfig logger = new LoggerConfig(
            "api", LogLevel.WARN, true,
            new String[]{"password", "token"}, 2
        )) {
            logger.log(LogLevel.DEBUG, "esto no se muestra (filtrado por minLevel)");
            logger.log(LogLevel.WARN,  "{\"user\":\"bob\",\"password\":\"hunter2\"}");
            logger.log(LogLevel.ERROR, "{\"code\":500,\"reason\":\"upstream timeout\"}");
        }

        // --- Singleton ---

        getLogger().log(LogLevel.INFO,  "app booted via singleton");
        getLogger().log(LogLevel.ERROR, "{\"error\":\"disk full\",\"password\":\"leaked?\"}");
    }
}

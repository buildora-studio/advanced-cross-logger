import com.crosslogger.LoggerConfig;
import com.crosslogger.LogLevel;
import java.util.UUID;

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
        String id = UUID.randomUUID().toString();

        // --- Uso básico (solo campos requeridos) ---

        try (LoggerConfig logger = new LoggerConfig("my-app", LogLevel.INFO)) {
            logger.log(LogLevel.SILLY, id, "starting trace");
            logger.log(LogLevel.DEBUG, id, "config loaded");
            logger.log(LogLevel.INFO,  id, "server ready on :8080");
            logger.log(LogLevel.WARN,  id, "memory usage above 80%");
            logger.log(LogLevel.ERROR, id, "failed to connect to db");
            logger.log(LogLevel.FATAL, id, "unrecoverable panic");
        }

        // --- Con isCloud ---

        try (LoggerConfig logger = new LoggerConfig("payments", LogLevel.INFO, true)) {
            logger.log(LogLevel.INFO, id, "service started");
        }

        // --- Con obfuscation ---

        try (LoggerConfig logger = new LoggerConfig(
            "auth", LogLevel.DEBUG, false,
            new String[]{"password", "token", "ssn"}, 2
        )) {
            logger.log(LogLevel.INFO, id, "{\"user\":\"alice\",\"password\":\"s3cr3t\"}");
            logger.log(LogLevel.WARN, id, "{\"session\":{\"token\":\"abc123\",\"expires\":3600}}");
        }

        // --- Con todo ---

        try (LoggerConfig logger = new LoggerConfig(
            "api", LogLevel.WARN, true,
            new String[]{"password", "token"}, 2
        )) {
            logger.log(LogLevel.DEBUG, id, "esto no se muestra (filtrado por minLevel)");
            logger.log(LogLevel.WARN,  id, "{\"user\":\"bob\",\"password\":\"hunter2\"}");
            logger.log(LogLevel.ERROR, id, "{\"code\":500,\"reason\":\"upstream timeout\"}");
        }

        // --- Singleton ---

        getLogger().log(LogLevel.INFO,  id, "app booted via singleton");
        getLogger().log(LogLevel.ERROR, id, "{\"error\":\"disk full\",\"password\":\"leaked?\"}");
    }
}

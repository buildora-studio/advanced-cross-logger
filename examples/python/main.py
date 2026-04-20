import json
from cross_logger import LoggerConfig, LogLevel

# --- Uso básico (solo campos requeridos) ---

logger = LoggerConfig("my-app", LogLevel.INFO)

logger.log(LogLevel.SILLY, "starting trace")
logger.log(LogLevel.DEBUG, "config loaded")
logger.log(LogLevel.INFO,  "server ready on :8080")
logger.log(LogLevel.WARN,  "memory usage above 80%")
logger.log(LogLevel.ERROR, "failed to connect to db")
logger.log(LogLevel.FATAL, "unrecoverable panic")

# --- Con is_cloud ---

cloud_logger = LoggerConfig("payments", LogLevel.INFO, is_cloud=True)
cloud_logger.log(LogLevel.INFO, "service started")

# --- Con obfuscation ---

secure_logger = LoggerConfig(
    "auth",
    LogLevel.DEBUG,
    obfuscate_keys=["password", "token", "ssn"],
    obfuscate_depth=2,
)

secure_logger.log(LogLevel.INFO, json.dumps({"user": "alice", "password": "s3cr3t"}))
secure_logger.log(LogLevel.WARN, json.dumps({"session": {"token": "abc123", "expires": 3600}}))

# --- Con todo ---

full_logger = LoggerConfig(
    "api",
    LogLevel.WARN,
    is_cloud=True,
    obfuscate_keys=["password", "token"],
    obfuscate_depth=2,
)

full_logger.log(LogLevel.DEBUG, "esto no se muestra (filtrado por min_level)")
full_logger.log(LogLevel.WARN,  json.dumps({"user": "bob", "password": "hunter2"}))
full_logger.log(LogLevel.ERROR, json.dumps({"code": 500, "reason": "upstream timeout"}))

# --- Singleton (patrón sugerido por binding) ---

_instance: LoggerConfig | None = None

def get_logger() -> LoggerConfig:
    global _instance
    if _instance is None:
        _instance = LoggerConfig(
            "global",
            LogLevel.INFO,
            obfuscate_keys=["password"],
            obfuscate_depth=2,
        )
    return _instance

get_logger().log(LogLevel.INFO, "app booted via singleton")
get_logger().log(LogLevel.ERROR, json.dumps({"error": "disk full", "password": "leaked?"}))

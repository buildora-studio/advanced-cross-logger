package main

import (
	"encoding/json"
	crosslogger "cross-logger"
	"github.com/google/uuid"
)

// Singleton pattern — cada desarrollador implementa el suyo.
var globalLogger *crosslogger.LoggerConfig

func getLogger() *crosslogger.LoggerConfig {
	if globalLogger == nil {
		globalLogger = crosslogger.NewLoggerConfig("global", crosslogger.LogLevelInfo,
			crosslogger.WithObfuscation([]string{"password", "token"}, 2),
		)
	}
	return globalLogger
}

func main() {
	id := uuid.New().String()

	// --- Uso básico (solo campos requeridos) ---

	logger := crosslogger.NewLoggerConfig("my-app", crosslogger.LogLevelInfo)
	defer logger.Close()

	logger.Log(crosslogger.LogLevelSilly, id, "starting trace")
	logger.Log(crosslogger.LogLevelDebug, id, "config loaded")
	logger.Log(crosslogger.LogLevelInfo,  id, "server ready on :8080")
	logger.Log(crosslogger.LogLevelWarn,  id, "memory usage above 80%")
	logger.Log(crosslogger.LogLevelError, id, "failed to connect to db")
	logger.Log(crosslogger.LogLevelFatal, id, "unrecoverable panic")

	// --- Con WithCloud ---

	cloud := crosslogger.NewLoggerConfig("payments", crosslogger.LogLevelInfo,
		crosslogger.WithCloud(true),
	)
	defer cloud.Close()
	cloud.Log(crosslogger.LogLevelInfo, id, "service started")

	// --- Con WithObfuscation ---

	secure := crosslogger.NewLoggerConfig("auth", crosslogger.LogLevelDebug,
		crosslogger.WithObfuscation([]string{"password", "token", "ssn"}, 2),
	)
	defer secure.Close()

	msg1, _ := json.Marshal(map[string]any{"user": "alice", "password": "s3cr3t"})
	secure.Log(crosslogger.LogLevelInfo, id, string(msg1))

	msg2, _ := json.Marshal(map[string]any{"session": map[string]any{"token": "abc123", "expires": 3600}})
	secure.Log(crosslogger.LogLevelWarn, id, string(msg2))

	// --- Con todo ---

	full := crosslogger.NewLoggerConfig("api", crosslogger.LogLevelWarn,
		crosslogger.WithCloud(true),
		crosslogger.WithObfuscation([]string{"password", "token"}, 2),
	)
	defer full.Close()

	full.Log(crosslogger.LogLevelDebug, id, "esto no se muestra (filtrado por minLevel)")

	msg3, _ := json.Marshal(map[string]any{"user": "bob", "password": "hunter2"})
	full.Log(crosslogger.LogLevelWarn, id, string(msg3))

	msg4, _ := json.Marshal(map[string]any{"code": 500, "reason": "upstream timeout"})
	full.Log(crosslogger.LogLevelError, id, string(msg4))

	// --- Singleton ---

	getLogger().Log(crosslogger.LogLevelInfo, id, "app booted via singleton")

	msg5, _ := json.Marshal(map[string]any{"error": "disk full", "password": "leaked?"})
	getLogger().Log(crosslogger.LogLevelError, id, string(msg5))
}

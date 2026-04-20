import { LoggerConfig, LogLevel } from '@cross-logger/node'

// --- Uso básico (solo campos requeridos) ---

const logger = new LoggerConfig('my-app', LogLevel.INFO)

logger.log(LogLevel.SILLY, 'starting trace')
logger.log(LogLevel.DEBUG, 'config loaded')
logger.log(LogLevel.INFO,  'server ready on :8080')
logger.log(LogLevel.WARN,  'memory usage above 80%')
logger.log(LogLevel.ERROR, 'failed to connect to db')
logger.log(LogLevel.FATAL, 'unrecoverable panic')

// --- Con opciones ---

const cloudLogger = new LoggerConfig('payments', LogLevel.INFO, {
  isCloud: true,
})

cloudLogger.log(LogLevel.INFO, 'service started')

// --- Con obfuscation ---

const secureLogger = new LoggerConfig('auth', LogLevel.DEBUG, {
  obfuscateKeys: ['password', 'token', 'ssn'],
  obfuscateDepth: 2,
})

secureLogger.log(LogLevel.INFO, JSON.stringify({ user: 'alice', password: 's3cr3t' }))
secureLogger.log(LogLevel.WARN, JSON.stringify({ session: { token: 'abc123', expires: 3600 } }))

// --- Con todo ---

const fullLogger = new LoggerConfig('api', LogLevel.WARN, {
  isCloud: true,
  obfuscateKeys: ['password', 'token'],
  obfuscateDepth: 2,
})

fullLogger.log(LogLevel.DEBUG, 'esto no se muestra (filtrado por minLevel)')
fullLogger.log(LogLevel.WARN,  JSON.stringify({ user: 'bob', password: 'hunter2' }))
fullLogger.log(LogLevel.ERROR, JSON.stringify({ code: 500, reason: 'upstream timeout' }))

// --- Singleton (patrón sugerido por binding) ---

let _instance: LoggerConfig | null = null

function getLogger(): LoggerConfig {
  if (!_instance) {
    _instance = new LoggerConfig('global', LogLevel.INFO, {
      isCloud: false,
      obfuscateKeys: ['password'],
      obfuscateDepth: 2,
    })
  }
  return _instance
}

getLogger().log(LogLevel.INFO, 'app booted via singleton')
getLogger().log(LogLevel.ERROR, JSON.stringify({ error: 'disk full', password: 'leaked?' }))

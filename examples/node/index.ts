import { LoggerConfig, LogLevel } from '@cross-logger/node'

const id = crypto.randomUUID()

// --- Uso básico (solo campos requeridos) ---

const logger = new LoggerConfig('my-app', LogLevel.INFO)

logger.log(LogLevel.SILLY, id, 'starting trace')
logger.log(LogLevel.DEBUG, id, 'config loaded')
logger.log(LogLevel.INFO,  id, 'server ready on :8080')
logger.log(LogLevel.WARN,  id, 'memory usage above 80%')
logger.log(LogLevel.ERROR, id, 'failed to connect to db')
logger.log(LogLevel.FATAL, id, 'unrecoverable panic')

// --- Con opciones ---

const cloudLogger = new LoggerConfig('payments', LogLevel.INFO, {
  isCloud: true,
})

cloudLogger.log(LogLevel.INFO, id, 'service started')

// --- Con obfuscation ---

const secureLogger = new LoggerConfig('auth', LogLevel.DEBUG, {
  obfuscateKeys: ['password', 'token', 'ssn'],
  obfuscateDepth: 2,
})

secureLogger.log(LogLevel.INFO, id, JSON.stringify({ user: 'alice', password: 's3cr3t' }))
secureLogger.log(LogLevel.WARN, id, JSON.stringify({ session: { token: 'abc123', expires: 3600 } }))

// --- Con todo ---

const fullLogger = new LoggerConfig('api', LogLevel.WARN, {
  isCloud: true,
  obfuscateKeys: ['password', 'token'],
  obfuscateDepth: 2,
})

fullLogger.log(LogLevel.DEBUG, id, 'esto no se muestra (filtrado por minLevel)')
fullLogger.log(LogLevel.WARN,  id, JSON.stringify({ user: 'bob', password: 'hunter2' }))
fullLogger.log(LogLevel.ERROR, id, JSON.stringify({ code: 500, reason: 'upstream timeout' }))

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

getLogger().log(LogLevel.INFO, id, 'app booted via singleton')
getLogger().log(LogLevel.ERROR, id, JSON.stringify({ error: 'disk full', password: 'leaked?' }))

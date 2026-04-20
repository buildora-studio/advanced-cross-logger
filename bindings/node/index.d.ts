/** Numeric severity levels. Pass these to LoggerConfig.log(). */
export declare const LogLevel: {
  readonly OFF:   -1
  readonly SILLY:  0
  readonly DEBUG:  1
  readonly INFO:   2
  readonly WARN:   3
  readonly ERROR:  4
  readonly FATAL:  5
}

/** Union of all valid level values. */
export type LogLevelValue = -1 | 0 | 1 | 2 | 3 | 4 | 5

/** Optional settings for LoggerConfig. */
export interface LoggerOptions {
  /** Cloud-compatible output format (no ANSI). Default: false. */
  isCloud?: boolean
  /** Keys whose values are replaced with "***" in JSON messages. */
  obfuscateKeys?: string[]
  /** Max nesting depth to search for obfuscateKeys. 0 disables obfuscation. */
  obfuscateDepth?: number
}

/**
 * Named logger instance. Create one per service or component.
 *
 * @example
 * const logger = new LoggerConfig('payments', LogLevel.INFO, {
 *   isCloud: true,
 *   obfuscateKeys: ['password', 'token'],
 *   obfuscateDepth: 2,
 * })
 * logger.log(LogLevel.WARN, JSON.stringify({ user: 'alice', password: 's3cr3t' }))
 */
export declare class LoggerConfig {
  /**
   * @param name     - Identifier shown in every log entry.
   * @param minLevel - Entries below this level are dropped. Use LogLevel constants.
   * @param options  - Optional: isCloud, obfuscateKeys, obfuscateDepth.
   */
  constructor(name: string, minLevel: LogLevelValue, options?: LoggerOptions)

  /**
   * Emits a log entry if level >= minLevel.
   * If message is a JSON string it is embedded as a nested object in the output.
   */
  log(level: LogLevelValue, message: string): void
}

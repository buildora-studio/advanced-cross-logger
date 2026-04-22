export interface LoggerOptions {
  isCloud?: boolean | null
  obfuscateKeys?: Array<string> | null
  obfuscateDepth?: number | null
}

export declare class LoggerConfig {
  constructor(name: string, minLevel: number, options?: LoggerOptions | null)
  log(level: number, id: string, message: string): void
}

export declare const LogLevel: {
  readonly OFF: -1
  readonly SILLY: 0
  readonly DEBUG: 1
  readonly INFO: 2
  readonly WARN: 3
  readonly ERROR: 4
  readonly FATAL: 5
}

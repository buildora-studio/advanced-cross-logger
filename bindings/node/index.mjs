import { createRequire } from 'module'

const require = createRequire(import.meta.url)
const native = require('./index.js')

export const LogLevel = native.LogLevel
export const LoggerConfig = native.LoggerConfig

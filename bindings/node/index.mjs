import { createRequire } from 'module'

const require = createRequire(import.meta.url)
const native = require('./index.js')

export const logInfo = native.logInfo
export const logWarning = native.logWarning
export const logError = native.logError

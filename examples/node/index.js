import pkg from '@cross-logger/node'
const { logInfo, logWarning, logError } = pkg

logInfo('Server started on port 3000')
logWarning('Memory usage above 80%')
logError('Failed to connect to database')

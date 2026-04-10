package main

import crosslogger "cross-logger"

func main() {
	crosslogger.LogInfo("Server started on port 3000")
	crosslogger.LogWarn("Memory usage above 80%")
	crosslogger.LogError("Failed to connect to database")
}

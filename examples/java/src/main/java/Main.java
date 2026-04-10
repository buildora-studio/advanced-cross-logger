import com.crosslogger.CrossLogger;

public class Main {
    public static void main(String[] args) {
        CrossLogger.logInfo("Server started on port 3000");
        CrossLogger.logWarn("Memory usage above 80%");
        CrossLogger.logError("Failed to connect to database");
    }
}

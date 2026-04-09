pub enum LogLevel {
    Info,
    Warn,
    Error,
}

pub fn print_log(level: LogLevel, message: &str) {
    match level {
        LogLevel::Info => println!("[INFO] {}", message),
        LogLevel::Warn => println!("[WARN] {}", message),
        LogLevel::Error => println!("[ERROR] {}", message),
    }
}
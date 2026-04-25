use core::{LoggerConfig, LogLevel};
use core::Uuid;

fn main() {
    let id = Uuid::new_v4();

    println!("=== Todos los niveles (terminal) ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Silly);
    logger.log(LogLevel::Silly, id, "starting trace");
    logger.log(LogLevel::Debug, id, "config loaded");
    logger.log(LogLevel::Info,  id, "server ready on :8080");
    logger.log(LogLevel::Warn,  id, "memory usage above 80%");
    logger.log(LogLevel::Error, id, "failed to connect to db");
    logger.log(LogLevel::Fatal, id, "unrecoverable panic");

    println!("\n=== Con is_cloud ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Debug)
        .with_cloud(true);
    logger.log(LogLevel::Info, id, "server ready on :8080");

    println!("\n=== Con obfuscation (depth 1) ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Debug)
        .with_obfuscation(vec!["password", "token"], 1);
    logger.log(LogLevel::Info, id, r#"{"user":"alice","password":"s3cr3t","token":"abc123"}"#);

    println!("\n=== Con obfuscation (depth 2) ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Debug)
        .with_obfuscation(vec!["password", "token"], 2);
    logger.log(LogLevel::Info, id, r#"{"session":{"token":"abc123","expires":3600}}"#);
    logger.log(LogLevel::Info, id, r#"{"data":{"user":{"password":"s3cr3t","name":"bob"}}}"#);

    println!("\n=== Con todo (cloud + obfuscation) ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Debug)
        .with_cloud(true)
        .with_obfuscation(vec!["password", "ssn"], 2);
    logger.log(LogLevel::Warn, id, r#"{"user":"alice","password":"s3cr3t","profile":{"ssn":"123-45-6789"}}"#);

    println!("\n=== min_level filtra entradas ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Warn);
    logger.log(LogLevel::Debug, id, "esto no se muestra");
    logger.log(LogLevel::Info,  id, "esto no se muestra");
    logger.log(LogLevel::Warn,  id, "memory usage above 80%");
    logger.log(LogLevel::Error, id, "failed to connect to db");
}

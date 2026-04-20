use core::{init, log, LoggerConfig, LogLevel};

fn main() {
    println!("=== Solo required (name + minLevel) ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Debug);
    logger.log(LogLevel::Info, "server ready on :8080");

    println!("\n=== Con is_cloud ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Debug)
        .with_cloud(true);
    logger.log(LogLevel::Info, "server ready on :8080");

    println!("\n=== Con obfuscation (depth 1) ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Debug)
        .with_obfuscation(vec!["password", "token"], 1);
    logger.log(LogLevel::Info, r#"{"user":"alice","password":"s3cr3t","token":"abc123"}"#);

    println!("\n=== Con obfuscation (depth 2) ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Debug)
        .with_obfuscation(vec!["password", "token"], 2);
    logger.log(LogLevel::Info, r#"{"session":{"token":"abc123","expires":3600}}"#);
    logger.log(LogLevel::Info, r#"{"data":{"user":{"password":"s3cr3t","name":"bob"}}}"#);

    println!("\n=== Con todo (cloud + obfuscation) ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Debug)
        .with_cloud(true)
        .with_obfuscation(vec!["password", "ssn"], 2);
    logger.log(LogLevel::Warn, r#"{"user":"alice","password":"s3cr3t","profile":{"ssn":"123-45-6789"}}"#);

    println!("\n=== min_level filtra entradas ===\n");

    let logger = LoggerConfig::new("my-app", LogLevel::Warn);
    logger.log(LogLevel::Debug, "esto no se muestra");
    logger.log(LogLevel::Info,  "esto no se muestra");
    logger.log(LogLevel::Warn,  "memory usage above 80%");
    logger.log(LogLevel::Error, "failed to connect to db");

    println!("\n=== Singleton global ===\n");

    init(
        LoggerConfig::new("global", LogLevel::Info)
            .with_cloud(true)
            .with_obfuscation(vec!["password"], 2),
    );

    log(LogLevel::Info, "app booted");
    log(LogLevel::Warn, r#"{"user":"alice","password":"s3cr3t"}"#);
    log(LogLevel::Debug, "esto no se muestra (filtrado por minLevel)");

    // segunda llamada a init es ignorada — el primero gana
    init(LoggerConfig::new("ignored", LogLevel::Silly));
    log(LogLevel::Info, "sigue usando 'global'");
}

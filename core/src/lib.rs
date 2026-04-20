use chrono::Utc;
use serde_json::{json, Value};

/// Severity levels for log entries, ordered from least to most critical.
///
/// The numeric value (`i8`) allows direct comparison for `min_level` filtering.
/// Use `Off` to silence all output.
///
/// # Levels
/// | Value | Variant | Use case |
/// |-------|---------|----------|
/// | -1 | `Off` | Disables all logging |
/// | 0 | `Silly` | Hyper-verbose tracing |
/// | 1 | `Debug` | Development diagnostics |
/// | 2 | `Info` | Normal operational events |
/// | 3 | `Warn` | Degraded but recoverable state |
/// | 4 | `Error` | Failures that need attention |
/// | 5 | `Fatal` | Unrecoverable errors |
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Off = -1,
    Silly = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

impl LogLevel {
    fn label(self) -> &'static str {
        match self {
            LogLevel::Off   => "OFF",
            LogLevel::Silly => "SILLY",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info  => "INFO",
            LogLevel::Warn  => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
}

/// Configuration for a named logger instance.
///
/// Each `LoggerConfig` represents an independent logger with its own name,
/// minimum level filter, and cloud flag. Multiple instances can coexist
/// with different settings (e.g. one per service or component).
///
/// # Output format
///
/// All output is JSON, one entry per line — compatible with CloudWatch,
/// GCP Logs Explorer, and any log aggregator that ingests structured logs:
///
/// ```text
/// {"timestamp":"...","severity":"INFO","logger":"my-app","message":"text or object"}
/// ```
///
/// If `message` is a valid JSON string, it is embedded as a nested object
/// rather than a plain string, enabling field-level filtering in log explorers.
///
/// # Example
///
/// ```rust
/// use core::{LoggerConfig, LogLevel};
///
/// let logger = LoggerConfig::new("payments", LogLevel::Info)
///     .with_cloud(true)
///     .with_obfuscation(vec!["password", "token"], 2);
///
/// logger.log(LogLevel::Info, r#"{"user":"alice","password":"s3cr3t"}"#);
/// // {"message":{"password":"***","user":"alice"},"severity":"INFO",...}
/// ```
pub struct LoggerConfig {
    /// Identifies the logger in every log entry (`"logger"` field).
    pub name: String,
    /// Entries below this level are silently dropped. `Off` suppresses everything.
    pub min_level: LogLevel,
    /// Reserved for cloud-specific behaviour (e.g. routing, extra metadata).
    /// Does not affect output format — both modes emit the same JSON.
    pub is_cloud: bool,
    /// Keys whose values are replaced with `"***"` before logging.
    /// Only applies when `message` is a valid JSON object or array.
    pub obfuscate_keys: Vec<String>,
    /// Maximum nesting depth to search for `obfuscate_keys`.
    /// Depth `1` covers only top-level keys; depth `2` includes one level of nesting, etc.
    /// `0` disables obfuscation entirely.
    pub obfuscate_depth: usize,
}

impl LoggerConfig {
    /// Creates a new logger with the required fields. Optional settings can be
    /// chained via [`with_cloud`](Self::with_cloud) and [`with_obfuscation`](Self::with_obfuscation).
    ///
    /// # Arguments
    ///
    /// * `name` — label included in every log entry
    /// * `min_level` — entries with a lower level are dropped; use `LogLevel::Off` to silence all
    pub fn new(name: impl Into<String>, min_level: LogLevel) -> Self {
        Self {
            name: name.into(),
            min_level,
            is_cloud: false,
            obfuscate_keys: vec![],
            obfuscate_depth: 0,
        }
    }

    /// Marks this logger as running in a cloud environment (CloudWatch, GCP, etc.).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::{LoggerConfig, LogLevel};
    /// let logger = LoggerConfig::new("api", LogLevel::Info)
    ///     .with_cloud(true);
    /// ```
    pub fn with_cloud(mut self, is_cloud: bool) -> Self {
        self.is_cloud = is_cloud;
        self
    }

    /// Enables obfuscation for the specified keys up to the given nesting depth.
    ///
    /// Matching key values are replaced with `"***"`. Only applies to JSON messages.
    ///
    /// # Arguments
    ///
    /// * `keys` — list of key names to redact
    /// * `depth` — how many levels deep to search (`1` = top-level only, `2` = one level nested, …)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::{LoggerConfig, LogLevel};
    /// let logger = LoggerConfig::new("auth", LogLevel::Info, false)
    ///     .with_obfuscation(vec!["password", "token", "ssn"], 3);
    /// ```
    pub fn with_obfuscation(mut self, keys: Vec<&str>, depth: usize) -> Self {
        self.obfuscate_keys = keys.into_iter().map(String::from).collect();
        self.obfuscate_depth = depth;
        self
    }

    /// Emits a log entry if `level` is not filtered by `min_level`.
    ///
    /// If `message` is a valid JSON string it is embedded as an object in the
    /// `"message"` field; otherwise it is stored as a plain string.
    /// Obfuscation is applied before output when `obfuscate_depth > 0`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::{LoggerConfig, LogLevel};
    /// let logger = LoggerConfig::new("api", LogLevel::Debug, false);
    ///
    /// // Plain string
    /// logger.log(LogLevel::Info, "request received");
    ///
    /// // JSON object — embedded as nested object in the output
    /// logger.log(LogLevel::Warn, r#"{"latency_ms":842,"route":"/checkout"}"#);
    /// ```
    pub fn log(&self, level: LogLevel, message: &str) {
        if self.min_level == LogLevel::Off || level < self.min_level {
            return;
        }
        let mut msg_value: Value = serde_json::from_str(message)
            .unwrap_or_else(|_| Value::String(message.to_string()));

        if self.obfuscate_depth > 0 && !self.obfuscate_keys.is_empty() {
            obfuscate(&mut msg_value, &self.obfuscate_keys, self.obfuscate_depth, 1);
        }

        let entry = json!({
            "timestamp": Utc::now().to_rfc3339(),
            "severity": level.label(),
            "logger": self.name,
            "message": msg_value,
        });
        println!("{}", entry);
    }
}

/// Recursively replaces values of matching keys with `"***"` up to `max_depth`.
fn obfuscate(value: &mut Value, keys: &[String], max_depth: usize, current_depth: usize) {
    match value {
        Value::Object(map) => {
            for (k, v) in map.iter_mut() {
                if keys.iter().any(|key| key == k) {
                    *v = Value::String("***".to_string());
                } else if current_depth < max_depth {
                    obfuscate(v, keys, max_depth, current_depth + 1);
                }
            }
        }
        Value::Array(arr) => {
            if current_depth < max_depth {
                for item in arr.iter_mut() {
                    obfuscate(item, keys, max_depth, current_depth);
                }
            }
        }
        _ => {}
    }
}

/// Emits a single log entry without a named logger instance.
///
/// Useful for one-off log calls where creating a [`LoggerConfig`] is unnecessary.
/// Supports the same JSON message embedding as [`LoggerConfig::log`].
///
/// # Example
///
/// ```rust
/// use core::{print_log, LogLevel};
///
/// print_log(LogLevel::Info, "app booted");
/// print_log(LogLevel::Error, r#"{"exit_code":1,"reason":"missing config"}"#);
/// ```
pub fn print_log(level: LogLevel, message: &str) {
    let msg_value: Value = serde_json::from_str(message)
        .unwrap_or_else(|_| Value::String(message.to_string()));

    let entry = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "severity": level.label(),
        "message": msg_value,
    });
    println!("{}", entry);
}

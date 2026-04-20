use napi_derive::napi;
use cross_logger_core::{LoggerConfig as CoreConfig, LogLevel as CoreLevel};

fn to_level(value: i32) -> CoreLevel {
    match value {
        -1 => CoreLevel::Off,
        0  => CoreLevel::Silly,
        1  => CoreLevel::Debug,
        3  => CoreLevel::Warn,
        4  => CoreLevel::Error,
        5  => CoreLevel::Fatal,
        _  => CoreLevel::Info,
    }
}

/// Options passed to LoggerConfig constructor and init().
#[napi(object)]
pub struct LoggerOptions {
    /// Route output through cloud-compatible format. Default: false.
    pub is_cloud: Option<bool>,
    /// Keys whose values are replaced with "***" in JSON messages.
    pub obfuscate_keys: Option<Vec<String>>,
    /// Max nesting depth to search for obfuscate_keys. 0 disables obfuscation.
    pub obfuscate_depth: Option<u32>,
}

fn build_core_config(name: String, min_level: i32, options: Option<LoggerOptions>) -> CoreConfig {
    let mut config = CoreConfig::new(name, to_level(min_level));
    if let Some(opts) = options {
        if let Some(is_cloud) = opts.is_cloud {
            config = config.with_cloud(is_cloud);
        }
        if let (Some(keys), Some(depth)) = (opts.obfuscate_keys, opts.obfuscate_depth) {
            let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
            config = config.with_obfuscation(key_refs, depth as usize);
        }
    }
    config
}

/// Named logger instance. Create one per service or component.
#[napi]
pub struct LoggerConfig {
    inner: CoreConfig,
}

#[napi]
impl LoggerConfig {
    /// Creates a logger with the given name and minimum log level.
    ///
    /// @param name - Identifier shown in every log entry.
    /// @param minLevel - Entries below this level are dropped. Use LogLevel constants.
    /// @param options - Optional: isCloud, obfuscateKeys, obfuscateDepth.
    #[napi(constructor)]
    pub fn new(name: String, min_level: i32, options: Option<LoggerOptions>) -> Self {
        Self { inner: build_core_config(name, min_level, options) }
    }

    /// Emits a log entry if level >= minLevel.
    ///
    /// @param level - One of the LogLevel constants.
    /// @param message - Plain string or JSON string. JSON is embedded as a nested object.
    #[napi]
    pub fn log(&self, level: i32, message: String) {
        self.inner.log(to_level(level), &message);
    }
}


use pyo3::prelude::*;
use cross_logger_core::{LoggerConfig as CoreConfig, LogLevel as CoreLevel, Uuid};

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

/// Named logger instance. Create one per service or component.
#[pyclass]
pub struct LoggerConfig {
    inner: CoreConfig,
}

#[pymethods]
impl LoggerConfig {
    /// Creates a logger with the given name and minimum log level.
    ///
    /// Args:
    ///     name: Identifier shown in every log entry.
    ///     min_level: Entries below this level are dropped. Use LogLevel constants.
    ///     is_cloud: Cloud-compatible output format. Default: False.
    ///     obfuscate_keys: Keys whose values are replaced with "***" in JSON messages.
    ///     obfuscate_depth: Max nesting depth to search for obfuscate_keys. Default: 0 (disabled).
    #[new]
    #[pyo3(signature = (name, min_level, is_cloud=false, obfuscate_keys=None, obfuscate_depth=0))]
    fn new(
        name: String,
        min_level: i32,
        is_cloud: bool,
        obfuscate_keys: Option<Vec<String>>,
        obfuscate_depth: usize,
    ) -> Self {
        let mut config = CoreConfig::new(name, to_level(min_level)).with_cloud(is_cloud);
        if let Some(keys) = obfuscate_keys {
            let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
            config = config.with_obfuscation(key_refs, obfuscate_depth);
        }
        Self { inner: config }
    }

    /// Emits a log entry if level >= min_level.
    /// If message is a JSON string it is embedded as a nested object.
    fn log(&self, level: i32, id: &str, message: &str) {
        let uuid = Uuid::parse_str(id).unwrap_or_default();
        self.inner.log(to_level(level), uuid, message);
    }
}

#[pymodule]
fn cross_logger(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LoggerConfig>()?;
    Ok(())
}

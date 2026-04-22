use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use cross_logger_core::{LoggerConfig as CoreConfig, LogLevel as CoreLevel, Uuid};

fn to_level(value: c_int) -> CoreLevel {
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

/// Creates a CoreConfig on the heap and returns its pointer.
///
/// # Safety
/// - `name` must be a valid non-null null-terminated C string.
/// - `obfuscate_keys` must be a valid array of `obfuscate_keys_count` non-null C strings,
///   or null when `obfuscate_keys_count` is 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_config_create(
    name: *const c_char,
    min_level: c_int,
    is_cloud: c_int,
    obfuscate_keys: *const *const c_char,
    obfuscate_keys_count: usize,
    obfuscate_depth: usize,
) -> *mut CoreConfig {
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap_or("") };
    let mut config = CoreConfig::new(name, to_level(min_level)).with_cloud(is_cloud != 0);

    if !obfuscate_keys.is_null() && obfuscate_keys_count > 0 {
        let keys_slice = unsafe { std::slice::from_raw_parts(obfuscate_keys, obfuscate_keys_count) };
        let keys: Vec<String> = keys_slice
            .iter()
            .map(|&p| unsafe { CStr::from_ptr(p).to_str().unwrap_or("").to_string() })
            .collect();
        let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
        config = config.with_obfuscation(key_refs, obfuscate_depth);
    }

    Box::into_raw(Box::new(config))
}

/// Emits a log entry through the native LoggerConfig handle.
///
/// # Safety
/// - `handle` must be a valid pointer returned by `logger_config_create`.
/// - `id` and `message` must be valid non-null null-terminated C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_config_log(
    handle: *mut CoreConfig,
    level: c_int,
    id: *const c_char,
    message: *const c_char,
) {
    let config = unsafe { &*handle };
    let id_str = unsafe { CStr::from_ptr(id).to_str().unwrap_or("") };
    let msg = unsafe { CStr::from_ptr(message).to_str().unwrap_or("") };
    let uuid = Uuid::parse_str(id_str).unwrap_or_default();
    config.log(to_level(level), uuid, msg);
}

/// Drops the CoreConfig behind the handle.
///
/// # Safety
/// `handle` must be a valid pointer returned by `logger_config_create` and must not
/// be used after this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_config_destroy(handle: *mut CoreConfig) {
    unsafe { drop(Box::from_raw(handle)) };
}

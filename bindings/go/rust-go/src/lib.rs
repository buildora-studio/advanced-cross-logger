use std::ffi::CStr;
use std::os::raw::c_char;
use cross_logger_core::{print_log, LogLevel};

/// # Safety
/// `message` must be a valid, non-null, null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_info(message: *const c_char) {
    let msg = unsafe { CStr::from_ptr(message).to_str().unwrap_or("") };
    print_log(LogLevel::Info, msg);
}

/// # Safety
/// `message` must be a valid, non-null, null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_warn(message: *const c_char) {
    let msg = unsafe { CStr::from_ptr(message).to_str().unwrap_or("") };
    print_log(LogLevel::Warn, msg);
}

/// # Safety
/// `message` must be a valid, non-null, null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_error(message: *const c_char) {
    let msg = unsafe { CStr::from_ptr(message).to_str().unwrap_or("") };
    print_log(LogLevel::Error, msg);
}

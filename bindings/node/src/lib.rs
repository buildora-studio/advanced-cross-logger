use napi_derive::napi;
use cross_logger_core::{print_log, LogLevel};

#[napi]
pub fn log_info(message: String) {
    print_log(LogLevel::Info, &message.to_string());
}

#[napi]
pub fn log_warning(message: String) {
    print_log(LogLevel::Warn, &message.to_string());
}

#[napi]
pub fn log_error(message: String) {
    print_log(LogLevel::Error, &message.to_string());
}
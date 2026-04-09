use pyo3::prelude::*;
use cross_logger_core::{print_log, LogLevel};

#[pyfunction]
fn log_info(message: &str) {
    print_log(LogLevel::Info, message);
}

#[pyfunction]
fn log_warn(message: &str) {
    print_log(LogLevel::Warn, message);
}

#[pyfunction]
fn log_error(message: &str) {
    print_log(LogLevel::Error, message);
}

#[pymodule]
fn cross_logger(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(log_info, m)?)?;
    m.add_function(wrap_pyfunction!(log_warn, m)?)?;
    m.add_function(wrap_pyfunction!(log_error, m)?)?;
    Ok(())
}
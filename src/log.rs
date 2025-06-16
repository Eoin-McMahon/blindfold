use colored::Colorize;
use std::io::{Result as IOResult, Write};

/// Logs a green "info" message.
pub fn log_info<T: Write>(msg: String, writer: &mut T) -> IOResult<()> {
    writeln!(writer, "{}", msg.green())
}

/// Logs a red "error" message.
pub fn log_error<T: Write>(msg: String, writer: &mut T) -> IOResult<()> {
    writeln!(writer, "{}", msg.red())
}

/// Logs a cyan "done" message.
pub fn log_done<T: Write>(msg: String, writer: &mut T) -> IOResult<()> {
    writeln!(writer, "{}", msg.cyan())
}

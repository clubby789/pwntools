//! A collection of logging utilities.
use crate::context;
use std::fmt::Display;

use colored::{ColoredString, Colorize};

/// Iterative log levels for filtering
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Silent = 4,
}

/// Generic logging function. Will not display logs of below the level specified in the `Context`.
/// # Arguments
///
/// * `message` - The message to be logged
/// * `level` - The `LogLevel` of the message.
///
/// # Examples
/// ```
/// use pwn::logging::{log, LogLevel};
/// log("Something went wrong", LogLevel::Error);
/// ```
pub fn log<T: Display>(message: T, level: LogLevel) {
    if level < context::get_loglevel() {
        return;
    }
    let log_char = match level {
        LogLevel::Debug => Some("|".purple()),
        LogLevel::Info => Some("*".blue()),
        LogLevel::Warn => Some("!".yellow()),
        LogLevel::Error => Some("X".red()),
        _ => None,
    };
    log_message(message, log_char);
}

fn log_message<T: Display>(message: T, char: Option<ColoredString>) {
    match char {
        Some(c) => println!("[{}] {}", c, message),
        None => println!("{}", message),
    }
}

/// Log a debug message
pub fn log_debug<T: Display>(message: T) {
    log(message, LogLevel::Debug);
}

/// Log an info message
pub fn log_info<T: Display>(message: T) {
    log(message, LogLevel::Info);
}

/// Log a warning
pub fn log_warn<T: Display>(message: T) {
    log(message, LogLevel::Warn);
}

/// Log an error
pub fn log_err<T: Display>(message: T) {
    log(message, LogLevel::Error);
}

use std::fmt::Display;
use crate::context::context;

extern crate colored;
use colored::Colorize;
use crate::logging::LogLevel::{Info, Warn, Error};

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Silent = 4
}

/// Generic logging function
/// Will not display logs of below the level specified in the `Context`.
/// # Arguments
///
/// * `message` - The message to be logged
/// * `level` - The `LogLevel` of the message.
///
/// # Examples
///
/// log("Something went wrong", LogLevel::Error);
pub fn log<T: Display>(message: T, level: LogLevel) {
    if level < context().log_level {
        return;
    }
    let log_char = match level {
        LogLevel::Debug => "|".purple(),
        LogLevel::Info => "*".blue(),
        LogLevel::Warn => "!".yellow(),
        LogLevel::Error => "X".red(),
        _ => " ".normal()
    };
    println!("[{}] {}", log_char, message);
}

/// Log an info message
pub fn log_info<T: Display>(message: T) {
    log(message, Info);
}

/// Log a warning
pub fn log_warn<T: Display>(message: T) {
    log(message, Warn);
}

/// Log an error
pub fn log_err<T: Display>(message: T) {
    log(message, Error);
}
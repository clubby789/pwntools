//! The global context. Used to set global settings which are used internally by several functions.
#![allow(dead_code)]
use crate::context::Bits::{SixtyFour, ThirtyTwo};
use crate::context::Endianness::Little;
use crate::logging::LogLevel;
use crate::logging::LogLevel::Info;

use once_cell::sync::Lazy;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// The word endianness of a given [`Arch`]
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Endianness {
    Little,
    Big,
}

/// The word size of a given [`Arch`]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Bits {
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
    SixtyFour = 64,
    OneTwentyEight = 128,
}

/// An architecture, identified by its endianness and word size
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Arch {
    pub endian: Endianness,
    pub bits: Bits,
}

/// The 64-bit version of x86
pub const AMD64: Arch = Arch {
    endian: Little,
    bits: SixtyFour,
};

/// The 32-bit version of x86
pub const I386: Arch = Arch {
    endian: Little,
    bits: ThirtyTwo,
};

/// The current context, used by most functions for runtime
/// behaviour modification
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Context {
    pub arch: Arch,
    pub log_level: LogLevel,
    pub endian: Endianness,
    pub bits: Bits,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            arch: I386,
            log_level: Info,
            endian: Little,
            bits: ThirtyTwo,
        }
    }
}

/** The default `Context`.
* Arch: [`I386`],
* Log Level: [`Info`],
* Endianness: [`Little`],
*  Bits: [`ThirtyTwo`]
**/
pub static CONTEXT: Lazy<RwLock<Context>> = Lazy::new(|| RwLock::new(Default::default()));

/// Retrieves a static reference to the global `Context`.
pub fn context() -> RwLockReadGuard<'static, Context> {
    CONTEXT.read().unwrap()
}

/// Retrieves a mutable static reference to the global `Context`.
pub fn context_mut() -> RwLockWriteGuard<'static, Context> {
    CONTEXT.write().unwrap()
}

impl Context {
    /// Set the context's architecture and update the bits/endianness correspondingly
    pub fn set_arch(&mut self, a: Arch) {
        self.arch = a;
        self.bits = a.bits;
        self.endian = a.endian;
    }
}

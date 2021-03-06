//! The global (to a given thread) context. Used to set global settings which are used internally by several functions.
//! # Examples
//! ```
//! use pwn::{context, I386};
//! use pwn::Bits::ThirtyTwo;
//! context::set_arch(I386);
//! assert_eq!(context::get_arch(), I386);
//! assert_eq!(context::get_bits(), ThirtyTwo)
//! ```
//! # Warning
//! `context` is local to each thread. Context values may need to be re-set when using multi-threaded code.
#![allow(dead_code)]

use crate::context::Bits::{SixtyFour, ThirtyTwo};
use crate::context::Endianness::Little;

use std::cell::RefCell;

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
    arch: Arch,
}

impl Default for Context {
    fn default() -> Self {
        Self { arch: I386 }
    }
}

thread_local! {
    /** The default `Context`.
    * Arch: [`I386`],
    * Log Level: [`Info`]
     **/
    static CONTEXT: RefCell<Context> = Default::default();
}

// Setters
/// Set the context's architecture
pub fn set_arch(a: Arch) {
    CONTEXT.with(|c| c.borrow_mut().arch = a)
}
/// Set the context's endianess
pub fn set_endianess(e: Endianness) {
    CONTEXT.with(|c| c.borrow_mut().arch.endian = e)
}
/// Set the context's word size
pub fn set_bits(b: Bits) {
    CONTEXT.with(|c| c.borrow_mut().arch.bits = b)
}
// Getters
/// Get the context's architecture
pub fn get_arch() -> Arch {
    CONTEXT.with(|c| c.borrow().arch)
}
/// Get the context's endianess
pub fn get_endianess() -> Endianness {
    CONTEXT.with(|c| c.borrow().arch.endian)
}
/// Get the context's word size
pub fn get_bits() -> Bits {
    CONTEXT.with(|c| c.borrow().arch.bits)
}

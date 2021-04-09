//! The global context. Used to set global settings which are used internally by several functions.

#![allow(dead_code)]
use crate::context::Bits::{Eight, Sixteen, SixtyFour, ThirtyTwo};
use crate::context::Endianness::{Big, Little};
use crate::logging::LogLevel;
use crate::logging::LogLevel::Info;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Bits {
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
    SixtyFour = 64,
    OneTwentyEight = 128,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Arch {
    pub endian: Endianness,
    pub bits: Bits,
}

pub const AARCH64: Arch = Arch {
    endian: Little,
    bits: SixtyFour,
};
pub const ALPHA: Arch = Arch {
    endian: Little,
    bits: SixtyFour,
};
pub const AVR: Arch = Arch {
    endian: Little,
    bits: Eight,
};
pub const AMD64: Arch = Arch {
    endian: Little,
    bits: SixtyFour,
};
pub const ARM: Arch = Arch {
    endian: Little,
    bits: ThirtyTwo,
};
pub const CRIS: Arch = Arch {
    endian: Little,
    bits: ThirtyTwo,
};
pub const I386: Arch = Arch {
    endian: Little,
    bits: ThirtyTwo,
};
pub const IA64: Arch = Arch {
    endian: Big,
    bits: SixtyFour,
};
pub const M68K: Arch = Arch {
    endian: Big,
    bits: ThirtyTwo,
};
pub const MIPS: Arch = Arch {
    endian: Little,
    bits: ThirtyTwo,
};
pub const MIPS64: Arch = Arch {
    endian: Little,
    bits: SixtyFour,
};
pub const MSP430: Arch = Arch {
    endian: Little,
    bits: Sixteen,
};
pub const POWERPC: Arch = Arch {
    endian: Big,
    bits: ThirtyTwo,
};
pub const POWERPC64: Arch = Arch {
    endian: Big,
    bits: SixtyFour,
};
pub const S390: Arch = Arch {
    endian: Big,
    bits: ThirtyTwo,
};
pub const SPARC: Arch = Arch {
    endian: Big,
    bits: ThirtyTwo,
};
pub const SPARC64: Arch = Arch {
    endian: Big,
    bits: SixtyFour,
};
pub const THUMB: Arch = Arch {
    endian: Little,
    bits: ThirtyTwo,
};
pub const VAX: Arch = Arch {
    endian: Little,
    bits: ThirtyTwo,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Context {
    pub arch: Arch,
    pub log_level: LogLevel,
    pub endian: Endianness,
    pub bits: Bits,
}

/** The default `Context`.
* Arch: [`I386`],
* Log Level: [`Info`],
* Endianness: [`Little`],
*  Bits: [`ThirtyTwo`]
**/
pub static mut CONTEXT: Context = Context {
    arch: I386,
    log_level: Info,
    endian: Little,
    bits: ThirtyTwo,
};

/// Retrieves a mutable static reference to the global `Context`.
///
/// # Safety
/// Mutable global statics are unsafe, so this provides a safe wrapper around it.
/// Currently, thread safety is not implemented.
pub fn context() -> &'static mut Context {
    unsafe { &mut CONTEXT }
}

impl Context {
    /// Set the context's architecture and update the bits/endianness correspondingly
    pub fn set_arch(&mut self, a: Arch) {
        context().arch = a;
        context().bits = a.bits;
        context().endian = a.endian;
    }
}

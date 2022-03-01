//! The global context. Used to set global settings which are used internally by several functions.
#![allow(dead_code)]
use crate::context::Bits::{Eight, Sixteen, SixtyFour, ThirtyTwo};
use crate::context::Endianness::{Big, Little};
use crate::logging::LogLevel;
use crate::logging::LogLevel::Info;

use once_cell::sync::Lazy;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

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

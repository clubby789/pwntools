//! Utilities for parsing ELF files.
//! # Examples
//! ## Loading from a file
//! ELF files are memory-mapped into the process, reducing their memory footprint.
//! ```no_run
//! use pwn::Elf;
//! let e = Elf::new("/tmp/binary");
//! dbg!(e.got().get("puts"));
//! ```
//!
//! ```no_run
//! use pwn::Elf;
//! let e = Elf::from_bytes(b"\x7fELF...");
//! dbg!(e.got().get("puts"));
//! ```
//! Symbols in the ELF, GOT and PLT are lazily resolved on request, reducing initial load delay.

#[allow(clippy::module_inception)]
mod elf;
pub use elf::Elf;

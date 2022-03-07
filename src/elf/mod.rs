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
//!
//! ELF files can also have their load address changed, which will rebase all their values.
//! ```no_run
//! use pwn::Elf;
//! let mut e = Elf::from_bytes(b"\x7fELF...");
//! // Relocatable files have their address set to 0
//! assert_eq!(e.address(), 0);
//! let orig = *e.got().get("puts").unwrap();
//! e.set_address(0x5000);
//! assert_eq!(*e.got().get("puts").unwrap(), orig+0x5000);
//! ```

#[allow(clippy::module_inception)]
mod elf;
pub use elf::Elf;

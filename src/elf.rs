use elf_utilities;
use elf_utilities::file::ELF::{ELF32, ELF64};
use elf_utilities::section::{Contents64,Contents32};

use std::fmt;
use std::fmt::Display;

/// A generic ELF struct for locating symbols
pub struct Elf {
    /// The path the ELF was loaded from.
    pub path: String,
    /// The base address of the ELF.
    pub address: u64,
    /// A dictionary of symbols to addresses.
    pub sym: Symtab,
    /// Whether PIE is enabled.
    pub pie: bool
    // TODO: GOT/PLT loading
    // pub got: Symtab,
    // pub plt: Symtab,
}

enum MixedElf {
    Elf32(elf_utilities::file::ELF32),
    Elf64(elf_utilities::file::ELF64),
}


impl Elf {
    /// Create an `Elf` from a file path.
    pub fn new<T: Into<String>>(path: T) -> Self {
        let path = path.into();
        let elf = elf_utilities::parser::parse_elf(&path).unwrap();
        let pie: bool;
        let elf = match elf {
            ELF32(e) => {
                pie = e.ehdr.e_type == 3;
                MixedElf::Elf32(e)
            },
            ELF64(e) => {
                pie = e.ehdr.e_type == 3;
                MixedElf::Elf64(e)
            },
        };

        let sym = Symtab::new(Elf::populate_sym(&elf));
        Elf {
            path,
            address: 0,
            sym,
            pie
            // got: Symtab::new(),
            // plt: Symtab::new(),
        }
    }

    /// Extract symbols from an Elf
    ///
    /// This has been moved into a separate function due to the
    /// repeated code involved with handling the ELF variants.
    fn populate_sym(elf: &MixedElf) -> Vec<Symbol> {
        let mut symbols: Vec<Symbol> = Vec::new();
        match &elf {
            MixedElf::Elf64(e) => {
                for section in &e.sections {
                    match &section.contents {
                        Contents64::Symbols(syms) => {
                            for sym in syms {
                                if sym.symbol_name != "" && sym.st_value != 0 {
                                    symbols.push(Symbol {
                                        name: sym.symbol_name.clone(),
                                        address: sym.st_value,
                                    });
                                }
                            }
                        },
                        _ => (),
                    }
                }
            },
            MixedElf::Elf32(e) => {
                for section in &e.sections {
                    match &section.contents {
                        Contents32::Symbols(syms) => {
                            for sym in syms {
                                if sym.symbol_name != "" && sym.st_value != 0 {
                                    symbols.push(Symbol {
                                        name: sym.symbol_name.clone(),
                                        address: sym.st_value as u64,
                                    });
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        symbols
    }
}

/// An ELF symbol.
#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub address: u64,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\": {}", self.name, self.address)
    }
}

/// A collection of [`Symbols`] from the binary.
pub struct Symtab {
    symbols: Vec<Symbol>,
}

impl Symtab {
    pub fn new(symbols: Vec<Symbol>) -> Self {
        Symtab { symbols }
    }
}

impl fmt::Display for Symtab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.symbols.iter().peekable();
        write!(f, "{{")?;
        while let Some(sym) = iter.next() {
            write!(f, "{}", sym)?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl<Q> std::ops::Index<Q> for Symtab
where
    Q: Display,
{
    type Output = Symbol;
    fn index(&self, name: Q) -> &Self::Output {
        let matches = &self
            .symbols
            .iter()
            .filter(|s| s.name == format!("{}", name))
            .collect::<Vec<&Symbol>>();
        &matches[0]
    }
}

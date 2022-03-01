//! A module for dealing with ELF files
use std::collections::HashMap;
use std::path::PathBuf;

use goblin::elf::header::{ET_DYN, ET_EXEC};
use goblin::elf::program_header::{PT_INTERP, PT_LOAD};
use goblin::elf::section_header::SHN_UNDEF;
use goblin::elf::Elf as GoblinElf;
use once_cell::sync::OnceCell;

/// Wrapper around [`goblin::elf::Elf`]
pub struct Elf<'a> {
    path: PathBuf,
    elf: GoblinElf<'a>,
    symbols: OnceCell<HashMap<&'a str, usize>>,
    got: OnceCell<HashMap<&'a str, usize>>,
    plt: OnceCell<HashMap<&'a str, usize>>,
    statically_linked: bool,
    #[allow(dead_code)]
    address: usize,
}

impl<'a> Elf<'a> {
    /// Create a new [`Elf`] loaded from a path
    ///
    /// *Note*: Due to a implementation detail, ELF files loaded have their memory leaked,
    /// so be careful of repeated loads.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let mapped = Box::new(unsafe {
            memmap::MmapOptions::new().map(&std::fs::File::open(&path).expect("Could not open file"))
        }.expect("Could not mmap file"));
        let mapped = Box::leak(mapped);
        let internal = GoblinElf::parse(mapped).expect("Not a valid ELF file");
        let mut load_address = 0;
        if internal.header.e_type != ET_DYN {
            internal
                .program_headers
                .iter()
                .filter(|seg| seg.p_type == PT_LOAD)
                .for_each(|seg| {
                    let addr = seg.p_vaddr;
                    if addr != 0 && (addr < load_address || load_address == 0) {
                        load_address = addr;
                    }
                });
        }
        let mut statically_linked = internal.header.e_type == ET_EXEC && load_address != 0;
        if internal
            .program_headers
            .iter()
            .any(|seg| seg.p_type == PT_INTERP)
        {
            statically_linked = false;
        }

        Self {
            path,
            elf: internal,
            symbols: Default::default(),
            got: Default::default(),
            plt: Default::default(),
            statically_linked,
            address: load_address as usize,
        }
    }

    /// The path the ELF file was originally loaded from
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    /// The word size of the ELF file
    pub fn bits(&self) -> usize {
        if self.elf.is_64 {
            64
        } else {
            32
        }
    }

    /// A name->address mapping of the symbols in the ELF
    pub fn symbols(&self) -> &HashMap<&'a str, usize> {
        self.symbols.get_or_init(|| self.populate_symbols())
    }
    // Used to lazily populate the symbols map
    fn populate_symbols(&self) -> HashMap<&'a str, usize> {
        let mut syms = HashMap::new();
        for sym in &self.elf.syms {
            if sym.st_value == 0 {
                continue;
            }
            let name = self.elf.strtab.get_at(sym.st_name).unwrap_or("");
            if name.is_empty() {
                continue;
            }
            syms.insert(name, sym.st_value as usize);
        }
        for sym in &self.elf.dynsyms {
            if sym.st_value == 0 {
                continue;
            }
            let name = self.elf.dynstrtab.get_at(sym.st_name).unwrap_or("");
            if name.is_empty() {
                continue;
            }
            syms.insert(name, sym.st_value as usize);
        }
        for (name, addr) in self.plt() {
            if !syms.contains_key(name) {
                syms.insert(name, *addr);
            }
        }
        for (name, addr) in self.got() {
            if !syms.contains_key(name) {
                syms.insert(name, *addr);
            }
        }
        syms
    }

    /// A name->address mapping of the GOT entries in the ELF
    pub fn got(&self) -> &HashMap<&'a str, usize> {
        self.got.get_or_init(|| self.populate_got())
    }
    // Used to lazily populate the GOT map
    fn populate_got(&self) -> HashMap<&'a str, usize> {
        if self.statically_linked {
            return Default::default();
        }
        let mut got = HashMap::new();
        for (idx, sec) in &self.elf.shdr_relocs {
            let shdr = &self.elf.section_headers[*idx];
            if shdr.sh_link == SHN_UNDEF {
                continue;
            }
            for reloc in sec.iter() {
                let sym = reloc.r_sym;
                if sym == 0 {
                    continue;
                }
                if let Some(sym) = self.elf.dynsyms.get(sym) {
                    if let Some(name) = self.elf.dynstrtab.get_at(sym.st_name) {
                        if name.is_empty() || reloc.r_offset == 0 {
                            continue;
                        }
                        got.insert(name, reloc.r_offset as usize);
                    }
                }
            }
        }
        got
    }

    /// A name->address mapping of the PLT entries in the ELF
    pub fn plt(&self) -> &HashMap<&'a str, usize> {
        self.plt.get_or_init(|| self.populate_plt())
    }
    // Used to lazily populate the PLT map
    fn populate_plt(&self) -> HashMap<&'a str, usize> {
        if self.statically_linked || self.got().is_empty() {
            return Default::default();
        }
        let plt_section = self
            .elf
            .section_headers
            .iter()
            .find(|shdr| matches!(self.elf.shdr_strtab.get_at(shdr.sh_name), Some(".plt")))
            .expect("Not .plt section");

        let mut plt = HashMap::new();
        for (i, reloc) in self.elf.pltrelocs.iter().enumerate() {
            if let Some(sym) = self.elf.dynsyms.get(reloc.r_sym) {
                if let Some(name) = self.elf.dynstrtab.get_at(sym.st_name) {
                    plt.insert(
                        name,
                        (plt_section.sh_addr + (plt_section.sh_entsize * (i + 1) as u64)) as usize,
                    );
                }
            }
        }
        plt
    }
}

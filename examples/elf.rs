use pwn::{Elf, log_info};

fn main() {
    let e = Elf::new("/tmp/binary");
    log_info("Symbols:");
    for (&name, addr) in e.symbols().iter().filter(|(&n, _)| !n.contains("__")) {
        log_info(format!("{name:<30}: {addr:#012x}"));
    }
}
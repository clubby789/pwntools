use pwn::*;

fn main() {
    init_logger();
    let mut e = Elf::new("/tmp/binary");
    info!("Symbols:");
    for (&name, addr) in e.symbols().iter().filter(|(&n, _)| !n.contains("__")) {
        info!("{name:<30}: {addr:#012x}");
    }
    e.set_address(0x5000);
}

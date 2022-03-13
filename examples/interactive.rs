use pwn::*;
use std::io;

fn main() -> io::Result<()> {
    init_logger();
    let mut sock = Remote::new("tcpbin.com", 4242)?;
    sock.interactive()
}

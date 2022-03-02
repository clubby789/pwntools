use std::io;
use pwn::tubes::remote::Remote;
use pwn::tubes::tube::Tube;

fn main() -> io::Result<()> {
    let mut sock = Remote::new("tcpbin.com", 4242);
    sock.interactive()
}

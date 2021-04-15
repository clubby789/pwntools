use pwn::tubes::remote::Remote;
use pwn::tubes::tube::Tube;

fn main() {
    let mut sock = Remote::remote("tcpbin.com", 4242);
    sock.interactive();
}

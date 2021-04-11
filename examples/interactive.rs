use pwntools;
use pwntools::tubes::remote::Remote;
use pwntools::tubes::tube::Tube;

fn main() {
    let mut sock = Remote::remote("tcpbin.com", 4242);
    sock.interactive();
}

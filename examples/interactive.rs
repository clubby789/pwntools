use pwntools_rs::tubes::remote::Remote;
use pwntools_rs::tubes::tube::Tube;

fn main() {
    let mut sock = Remote::remote("tcpbin.com", 4242);
    sock.interactive();
}

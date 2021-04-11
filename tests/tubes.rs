use pwntools_rs::tubes::listen::Listen;
use pwntools_rs::tubes::remote::Remote;
use pwntools_rs::tubes::tube::Tube;
use std::time::Duration;

/// Test ability to write to a TCP echo server and read the result.
#[test]
fn echo_sock() {
    // TCP echo server
    let mut sock = Remote::remote("tcpbin.com", 4242);
    let data = b"test";
    sock.sendline(*data);
    let returned: &[u8] = &sock.recv();
    // Cut out the returned newline
    assert_eq!(returned[..4], data[..])
}

/// Test opening a listening socket and sending data to it
#[test]
fn listen_sock() {
    let mut listener = Listen::listen(Some("0.0.0.0"), None);
    let addr = listener.addr;
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(1));
        let mut sock = Remote::remote("127.0.0.1", addr.port());
        sock.send(*b"test");
    });
    assert_eq!(listener.recv(), b"test");
}

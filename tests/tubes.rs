use pwn::*;
use std::time::Duration;

/// Test ability to write to a TCP echo server and read the result.
#[test]
fn echo_sock() {
    // TCP echo server
    let mut sock = Remote::new("tcpbin.com", 4242).unwrap();
    let data = b"test";
    sock.sendline(*data).unwrap();
    let returned: &[u8] = &sock.recv().unwrap();
    // Cut out the returned newline
    assert_eq!(returned[..4], data[..])
}

/// Test opening a listening socket and sending data to it.
#[test]
fn listen_sock() {
    let mut listener = Listen::new(Some("0.0.0.0"), None).unwrap();
    let addr = listener.addr;
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(1));
        let mut sock = Remote::new("127.0.0.1", addr.port()).unwrap();
        sock.send(*b"test").unwrap();
    });
    assert_eq!(listener.recv().unwrap(), b"test");
}

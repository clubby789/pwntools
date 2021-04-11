use crate::logging::log;
use crate::logging::LogLevel::Info;
use crate::tubes::buffer::Buffer;
use crate::tubes::sock::Sock;
use crate::tubes::tube::Tube;
use std::net::TcpStream;
use std::time::Duration;

/// A generic TCP client struct
///
/// # Examples
/// ```
/// use pwntools::tubes::remote::Remote;
/// use pwntools::tubes::tube::Tube;
/// let mut sock = Remote::remote("tcpbin.com", 4242);
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
    pub fn remote<T: ToString, T2: Into<i32>>(host: T, port: T2) -> Remote {
        let port = port.into();
        let conn_str = format!("{}:{}", host.to_string(), port);
        log(format!("Opening connection to {}", conn_str), Info);
        Remote {
            sock: Sock::new(TcpStream::connect(conn_str).expect("Could not connect")),
            _host: host.to_string(),
            _port: port,
        }
    }
}

impl Tube for Remote {
    fn get_buffer(&mut self) -> &mut Buffer {
        self.sock.get_buffer()
    }

    fn fill_buffer(&mut self, timeout: Option<Duration>) -> usize {
        self.sock.fill_buffer(timeout)
    }

    fn send_raw(&mut self, data: Vec<u8>) {
        self.sock.send(data)
    }

    fn close(&mut self) {
        self.sock.close();
    }
}

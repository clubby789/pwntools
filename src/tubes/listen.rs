use std::io;
use crate::tubes::buffer::Buffer;
use crate::tubes::sock::Sock;
use crate::tubes::tube::Tube;
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;

/// A TCP listener which is connected to a [`Sock`]
///
/// # Example
/// Listen on all interfaces, on an OS-selected TCP port
/// ```
/// use pwn::tubes::listen::Listen;
/// let mut listener = Listen::new(Some("0.0.0.0"), None);
/// ```
pub struct Listen {
    listener: TcpListener,
    sock: Option<Sock>,
    pub addr: SocketAddr,
}

impl Listen {
    /// Create a TCP listener. By default, it will listen on all interfaces, and
    /// a port randomly chosen by the OS.
    pub fn new<T: ToString>(host: Option<T>, port: Option<i32>) -> Self {
        let host = match host {
            Some(h) => h.to_string(),
            None => "0.0.0.0".to_string(),
        };
        let port = match port {
            Some(p) => format!("{}", p),
            None => "0".to_string(),
        };

        let listener = TcpListener::bind(format!("{}:{}", host, port)).expect("Could not bind");
        let addr = listener.local_addr().expect("Could not get sock address");
        Listen {
            listener,
            sock: None,
            addr,
        }
    }

    /// Retrieve the internal `SocketAddr` of the listener.
    pub fn addr(&self) -> SocketAddr {
        self.listener
            .local_addr()
            .expect("Could not get bound address")
    }
}

impl Tube for Listen {
    /// Retrieve a mutable reference to the [`Sock`]'s internal [`Buffer`]. On first call,
    /// will block until a connection is received.
    fn get_buffer(&mut self) -> &mut Buffer {
        if self.sock.is_none() {
            self.sock = Some(Sock::new(
                self.listener
                    .accept()
                    .expect("Could not accept connection")
                    .0,
            ));
        }
        self.sock.as_mut().unwrap().get_buffer()
    }

    /// Fill the [`Sock`]'s internal [`Buffer`]. On first call, will block until
    /// a connection is received.
    fn fill_buffer(&mut self, timeout: Option<Duration>) -> io::Result<usize> {
        if self.sock.is_none() {
            self.sock = Some(Sock::new(
                self.listener
                    .accept()
                    .expect("Could not accept connection")
                    .0,
            ));
        }
        self.sock.as_mut().unwrap().fill_buffer(timeout)
    }

    /// Send a message via the [`Sock`]. On first call, will block until
    /// a connection is received.
    fn send_raw(&mut self, data: Vec<u8>) -> io::Result<()> {
        if self.sock.is_none() {
            self.sock = Some(Sock::new(
                self.listener
                    .accept()
                    .expect("Could not accept connection")
                    .0,
            ));
        }
        self.sock.as_mut().unwrap().send(data)
    }

    /// Close the internal [`Sock`].
    fn close(&mut self) {
        if let Some(sock) = &mut self.sock {
            sock.close();
        }
    }
}

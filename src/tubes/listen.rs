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
/// use pwntools_rs::tubes::listen::Listen;
/// let mut listener = Listen::listen(Some("0.0.0.0"), None);
/// ```
pub struct Listen {
    listener: TcpListener,
    sock: Option<Sock>,
    pub addr: SocketAddr,
}

impl Listen {
    /// Create a TCP listener. By default, it will listen on all interfaces, and
    /// a port randomly chosen by the OS.
    pub fn listen<T: ToString>(host: Option<T>, port: Option<i32>) -> Self {
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
    fn get_buffer(&mut self) -> &mut Buffer {
        if let None = self.sock {
            self.sock = Some(Sock::new(
                self.listener
                    .accept()
                    .expect("Could not accept connection")
                    .0,
            ));
        }
        self.sock.as_mut().unwrap().get_buffer()
    }

    fn fill_buffer(&mut self, timeout: Option<Duration>) -> usize {
        if let None = self.sock {
            self.sock = Some(Sock::new(
                self.listener
                    .accept()
                    .expect("Could not accept connection")
                    .0,
            ));
        }
        self.sock.as_mut().unwrap().fill_buffer(timeout)
    }

    fn send_raw(&mut self, data: Vec<u8>) {
        if let None = self.sock {
            self.sock = Some(Sock::new(
                self.listener
                    .accept()
                    .expect("Could not accept connection")
                    .0,
            ));
        }
        self.sock.as_mut().unwrap().send(data)
    }

    fn close(&mut self) {
        if let Some(sock) = &mut self.sock {
            sock.close();
        }
    }
}

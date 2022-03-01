use crate::tubes::buffer::Buffer;
use crate::tubes::tube::Tube;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

/// A generic TCP socket that can be a client or server.
pub struct Sock {
    sock: TcpStream,
    buffer: Buffer,
}

impl Sock {
    /// Create a `Sock` from a `TcpStream` with an internal [`Buffer`].
    pub fn new(sock: TcpStream) -> Self {
        Self {
            sock,
            buffer: Buffer::new(),
        }
    }
}

impl Tube for Sock {
    /// Get a mutable reference to the internal [`Buffer`].
    fn get_buffer(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    /// Attempt to fill the internal [`Buffer`] with a given timeout.
    fn fill_buffer(&mut self, timeout: Option<Duration>) -> usize {
        self.sock.set_read_timeout(timeout).unwrap();
        let mut temp_buf: [u8; 1024] = [0; 1024];
        let mut total: usize = 0;
        loop {
            let read = self
                .sock
                .read(&mut temp_buf);
            let buffer = self.get_buffer();
            if let Ok(sz) = read {
                buffer.add(temp_buf[..sz].to_vec());
                total += sz;
                if sz < 1024 {
                    break;
                }
            } else {
                break;
            }
        }
        total
    }
    /// Send data via the [`Sock`].
    fn send_raw(&mut self, data: Vec<u8>) {
        self.sock.write_all(&data).expect("Could not write to socket");
    }

    /// Close the internal [`Sock`].
    fn close(&mut self) {
        self.sock
            .shutdown(Shutdown::Both)
            .expect("Could not shut down socket");
    }
}

impl Clone for Sock {
    fn clone(&self) -> Self {
        Sock {
            sock: self.sock.try_clone().unwrap(),
            buffer: self.buffer.clone()
        }
    }
}
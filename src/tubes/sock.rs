use crate::tubes::buffer::Buffer;
use crate::tubes::tube::Tube;
use std::io::{Read, Write};
use std::net::{TcpStream, Shutdown};

pub struct Sock {
    sock: TcpStream,
    buffer: Buffer,
}

impl Sock {
    pub fn new(sock: TcpStream) -> Self {
        Self {
            sock,
            buffer: Buffer::new(),
        }
    }
}

impl Tube for Sock {
    fn fill_buffer(&mut self) {
        let mut temp_buf: [u8; 1024] = [0; 1024];
        loop {
            let read = self
                .sock
                .read(&mut temp_buf)
                .expect("Could not read from socket");
            let buffer = self.get_buffer();
            buffer.add(temp_buf[..read].to_vec());
            if read < 1024 {
                break;
            }
        }
    }

    fn get_buffer(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    fn send_raw(&mut self, data: Vec<u8>) {
        self.sock.write(&data).expect("Could not write to socket");
    }

    fn close(&mut self) {
        self.sock.shutdown(Shutdown::Both).expect("Could not shut down socket");
    }
}

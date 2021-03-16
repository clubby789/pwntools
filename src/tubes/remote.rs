use crate::tubes::buffer::Buffer;
use crate::tubes::sock::Sock;
use crate::tubes::tube::Tube;
use std::net::TcpStream;

pub struct Remote {
    sock: Sock,
    _host: String,
    _port: i32,
}

impl Remote {
    pub fn remote<T: ToString, T2: Into<i32>>(host: T, port: T2) -> Self {
        let port = port.into();
        let conn_str = format!("{}:{}", host.to_string(), port);
        Remote {
            sock: Sock::new(TcpStream::connect(conn_str).expect("Could not connect")),
            _host: host.to_string(),
            _port: port,
        }
    }
}

impl Tube for Remote {
    fn fill_buffer(&mut self) {
        self.sock.fill_buffer();
    }

    fn get_buffer(&mut self) -> &mut Buffer {
        self.sock.get_buffer()
    }

    fn send_raw(&mut self, data: Vec<u8>) {
        self.sock.send(data)
    }

    fn close(&mut self) {
        self.sock.close();
    }
}

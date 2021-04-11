extern crate rustyline;
use rustyline::Editor;
extern crate crossbeam_utils;
use crossbeam_utils::thread;
use crate::logging::*;
use crate::tubes::buffer::Buffer;
use std::time::Duration;

/// Generic `Tube` trait
pub trait Tube {
    /// Retrieve mutable reference to the internal [`Buffer`].
    fn get_buffer(&mut self) -> &mut Buffer;
    /// Fill the internal [`Buffer`].
    ///
    /// * `timeout` - Maximum time to fill for. If `None`, block until data is read.
    fn fill_buffer(&mut self, timeout: Option<Duration>) -> usize;


    // Currently not working. Gives a timeout message.
    /// Retrieve all data from the `Tube`.
    ///
    /// * `timeout` - The maximum time to read for, defaults to 0.05s. If 0, clean only the
    /// internal buffer.
    fn clean(&mut self, timeout: Option<Duration>) -> Vec<u8> {
        let timeout = match timeout {
            Some(t) => t,
            None => Duration::from_millis(100),
        };
        if timeout == Duration::from_millis(0) {
            return self.get_buffer().get(0);
        }
        self.recvrepeat(Some(timeout))
    }


    /// Receives from the `Tube`, returning once any data is available.
    fn recv(&mut self) -> Vec<u8> {
        self._recv(None, None)
    }
    /// Receives `n` bytes from the `Tube`.
    fn recvn(&mut self, n: usize) -> Vec<u8> {
        self._recv(Some(n), None)
    }

    fn _recv(&mut self, numb: Option<usize>, timeout: Option<Duration>) -> Vec<u8> {
        self.fill_buffer(timeout);
        let numb = match numb {
            Some(sz) => sz,
            None => 0,
        };
        self.get_buffer().get(numb)
    }
    /// Receive all data from the `Tube`, repeatedly reading with the given `timeout`.
    fn recvrepeat(&mut self, timeout: Option<Duration>) -> Vec<u8> {
        while self.fill_buffer(timeout) > 0 {
            ();
        }
        self.get_buffer().get(0)
    }
    /// Writes data to the `Tube`.
    fn send<T: Into<Vec<u8>>>(&mut self, data: T) {
        let data = data.into();
        log_debug(format!("Sending {} bytes", data.len()));
        self.send_raw(data);
    }
    /// Appends a newline to the data before writing it to the `Tube`.
    fn sendline<T: Into<Vec<u8>>>(&mut self, data: T) {
        let mut data = data.into();
        data.push(b'\n');
        log_debug(format!("Sending {} bytes", data.len()));
        self.send_raw(data);
    }
    fn send_raw(&mut self, data: Vec<u8>);
    /// Close both ends of the `Tube`.
    fn close(&mut self);

    fn interactive(&mut self)
    where
        Self: Clone+Send,
    {
        let mut receiver = self.clone();
        // Make sure that the receiver thread does not outlive scope
        thread::scope(|s| {
            s.spawn(|_| {
                loop {
                    print!("{}", std::str::from_utf8(&receiver.clean(None)).unwrap());
                }
            });

        let mut rl = Editor::<()>::new();
        loop {
            if let Ok(line) = rl.readline("$ ") {
                self.sendline(line);
            } else {
                return;
            }
        }
        }).unwrap();
    }
}

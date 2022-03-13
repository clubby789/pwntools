extern crate rustyline;

use rustyline::Editor;
use std::io;
use std::io::Write;

extern crate crossbeam_utils;
use crate::debug;
use crate::tubes::buffer::Buffer;
use crossbeam_utils::thread;
use std::time::Duration;

/// Generic `Tube` trait, used as the underlying interface for IO.
pub trait Tube {
    /// Retrieve mutable reference to the internal [`Buffer`].
    fn get_buffer(&mut self) -> &mut Buffer;
    /// Fill the internal [`Buffer`].
    ///
    /// * `timeout` - Maximum time to fill for. If `None`, block until data is read.
    fn fill_buffer(&mut self, timeout: Option<Duration>) -> io::Result<usize>;

    // Currently not working. Gives a timeout message.
    /// Retrieve all data from the `Tube`.
    ///
    /// * `timeout` - The maximum time to read for, defaults to 0.05s. If 0, clean only the
    /// internal buffer.
    fn clean(&mut self, timeout: Duration) -> io::Result<Vec<u8>> {
        self.fill_buffer(Some(timeout))?;
        Ok(self.get_buffer().get(0))
    }

    /// Receives from the `Tube`, returning once any data is available.
    fn recv(&mut self) -> io::Result<Vec<u8>> {
        self.recv_raw(None, None)
    }
    /// Receives `n` bytes from the `Tube`.
    fn recvn(&mut self, n: usize) -> io::Result<Vec<u8>> {
        self.recv_raw(Some(n), None)
    }

    #[doc(hidden)]
    fn recv_raw(&mut self, numb: Option<usize>, timeout: Option<Duration>) -> io::Result<Vec<u8>> {
        self.fill_buffer(timeout)?;
        let numb = numb.unwrap_or(0);
        Ok(self.get_buffer().get(numb))
    }
    /// Receive all data from the `Tube`, repeatedly reading with the given `timeout`.
    fn recvrepeat(&mut self, timeout: Option<Duration>) -> io::Result<Vec<u8>> {
        while self.fill_buffer(timeout)? > 0 {}
        Ok(self.get_buffer().get(0))
    }
    /// Writes data to the `Tube`.
    fn send<T: Into<Vec<u8>>>(&mut self, data: T) -> io::Result<()> {
        let data = data.into();
        debug!("Sending {} bytes", data.len());
        self.send_raw(data)
    }
    /// Appends a newline to the data before writing it to the `Tube`.
    fn sendline<T: Into<Vec<u8>>>(&mut self, data: T) -> io::Result<()> {
        let mut data = data.into();
        data.push(b'\n');
        debug!("Sending {} bytes", data.len());
        self.send_raw(data)
    }

    #[doc(hidden)]
    fn send_raw(&mut self, data: Vec<u8>) -> io::Result<()>;
    /// Close both ends of the `Tube`.
    fn close(&mut self) -> io::Result<()>;

    /// Receive until the given delimiter is received.
    fn recvuntil(&mut self, delim: &[u8]) -> io::Result<Vec<u8>> {
        let mut pos;
        loop {
            self.fill_buffer(Some(Duration::from_millis(50)))?;
            pos = find_subsequence(self.get_buffer().data.make_contiguous(), delim);
            if let Some(p) = pos {
                return Ok(self.get_buffer().get(p + 1));
            }
        }
    }

    /// Receive from the tube until a newline is received.
    fn recvline(&mut self) -> io::Result<Vec<u8>> {
        self.recvuntil(b"\n")
    }
    /// Get an interactive prompt for the connection. A second thread will print messages as they
    /// arrive.
    fn interactive(&mut self) -> io::Result<()>
    where
        Self: Clone + Send,
    {
        let mut receiver = self.clone();
        // Make sure that the receiver thread does not outlive scope
        thread::scope(|s| {
            s.spawn(|_| loop {
                std::io::stdout()
                    .write_all(
                        &receiver
                            .clean(Duration::from_millis(50))
                            .unwrap_or_default(),
                    )
                    .expect("Couldn't write stdout")
            });

            let mut rl = Editor::<()>::new();
            loop {
                if let Ok(line) = rl.readline("$ ") {
                    if self.sendline(line).is_err() {
                        return;
                    }
                } else {
                    return;
                }
            }
        })
        .expect("Couldn't start receiving thread");
        Ok(())
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

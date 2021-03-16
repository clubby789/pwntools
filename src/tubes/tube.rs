use crate::tubes::buffer::Buffer;

/// Generic `Tube` trait
pub trait Tube {
    /// Receives up to `numb` bytes from the tube, returning as
    /// soon as any data is available
    fn recv(&mut self, numb: Option<usize>) -> Vec<u8> {
        self.fill_buffer();
        let buffer = self.get_buffer();
        if buffer.len() == 0 {
            return vec![];
        }
        buffer.get(numb)
    }
    fn send<T: Into<Vec<u8>>>(&mut self, data: T) {
        self.send_raw(data.into());
    }
    fn sendline<T: Into<Vec<u8>>>(&mut self, data: T) {
        let mut data = data.into().clone();
        data.push(b'\n');
        self.send(data);
    }
    /// Fill the buffer with available data
    fn fill_buffer(&mut self);
    /// Get the internal buffer
    fn get_buffer(&mut self) -> &mut Buffer;
    /// Sending data to the other end.
    fn send_raw(&mut self, data: Vec<u8>);
    /// Close both ends of the tube.
    fn close(&mut self);
}

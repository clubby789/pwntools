use std::collections::VecDeque;

/// List of strings with some helper routines
pub struct Buffer {
    /// A queue of `u8` (oldest is at the start)
    pub data: VecDeque<u8>,
    /// Total data size
    pub size: usize,
}

impl Buffer {
    /// Initialize a `Buffer`
    pub fn new() -> Self {
        Self {
            data: VecDeque::<u8>::new(),
            size: 0 as usize,
        }
    }
    /// Get the current `Buffer` length
    pub fn len(&self) -> usize {
        self.size
    }
    /// Adds new data to the buffer
    pub fn add(&mut self, data: Vec<u8>) {
        // TODO: Allow adding another buffer
        self.size += data.len();
        self.data.extend(data);
    }
    /// Retrieve max `size` bytes from the buffer. If `size` is `None`, get infinite.
    pub fn get(&mut self, size: Option<usize>) -> Vec<u8> {
        match size {
            Some(sz) if (sz > self.size || sz == 0) => self.data.drain(..).collect(),
            Some(sz) if sz <= self.size => self.data.drain(0..sz).collect(),
            _ => self.data.drain(..).collect(),
        }
    }
}

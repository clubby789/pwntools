use std::collections::VecDeque;

/// Collection of bytes with some helper routines.
///
/// Used as the backing store for the `Tube` structs.
#[derive(Clone)]
pub struct Buffer {
    /// A queue of `u8` (oldest is at the start)
    pub data: VecDeque<u8>,
    /// Total data size
    pub size: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    /// Initialize a `Buffer`.
    pub fn new() -> Self {
        Self {
            data: VecDeque::<u8>::new(),
            size: 0,
        }
    }
    /// Get the current `Buffer` length.
    pub fn len(&self) -> usize {
        self.size
    }
    /// Check if `Buffer` is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Adds new data to the buffer.
    pub fn add(&mut self, data: Vec<u8>) {
        // TODO: Allow adding another buffer
        self.size += data.len();
        self.data.extend(data);
    }
    /// Receive bytes from the buffer.
    ///
    /// * `numb` - The maximum bytes to retrieve. If 0, unlimited.
    pub fn get(&mut self, numb: usize) -> Vec<u8> {
        if numb > self.size || numb == 0 {
            self.data.drain(..).collect()
        } else if numb <= self.size {
            self.data.drain(0..numb).collect()
        } else {
            self.data.drain(..).collect()
        }
    }
    /// Place data at the front of the buffer.
    pub fn unget(&mut self, data: Vec<u8>) {
        self.size += data.len();
        for item in data.iter().rev() {
            self.data.push_back(*item);
        }
    }
}

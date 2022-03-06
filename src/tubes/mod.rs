//! Utilities for IO 'tubes'.

/// The internal byte buffer used by `Tube`s.
pub mod buffer;
pub use buffer::*;
/// A general-purpose TCP server.
pub mod listen;
pub use listen::*;
/// A general-purpose TCP client.
pub mod remote;
pub use remote::*;
/// A generic TCP socket.
pub mod sock;
pub use sock::*;
/// A generic IO 'tube'.
pub mod tube;
pub use tube::*;

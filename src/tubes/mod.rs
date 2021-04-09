//! Utilities for IO 'tubes'.

/// The internal byte buffer used by `Tube`s.
pub mod buffer;
/// A general-purpose TCP server.
pub mod listen;
/// A general-purpose TCP client.
pub mod remote;
/// A generic TCP socket.
pub mod sock;
/// A generic IO 'tube'.
pub mod tube;

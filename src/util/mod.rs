//! A collection of miscellaneous utilities.

/// Utilities for packing unsigned integers to `Vec<u8>`s of appropriate size.
pub mod packing;
pub use packing::*;
/// Utility for flattening a collection of data to bytes.
pub mod flat;
pub use flat::*;

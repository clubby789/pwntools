use crate::context::{
    self,
    Endianness::{Big, Little},
};
use crate::Bits;
use byteorder::{BigEndian, ByteOrder, LittleEndian};

/// Packs a `u8` to a 1-byte `Vec`.
pub fn p8(v: u8) -> Vec<u8> {
    vec![v]
}

/// Packs a `u16` to a 2-byte `Vec`.
pub fn p16(v: u16) -> Vec<u8> {
    let mut res: Vec<u8> = std::iter::repeat(0).take(2).collect();
    match context::get_endianess() {
        Big => BigEndian::write_u16(&mut res, v),
        Little => LittleEndian::write_u16(&mut res, v),
    };
    res
}

/// Packs a `u32` to a 4-byte `Vec`.
pub fn p32(v: u32) -> Vec<u8> {
    let mut res: Vec<u8> = std::iter::repeat(0).take(4).collect();
    match context::get_endianess() {
        Big => BigEndian::write_u32(&mut res, v),
        Little => LittleEndian::write_u32(&mut res, v),
    };
    res
}

/// Packs a `u64` to an 8-byte `Vec`.
pub fn p64(v: u64) -> Vec<u8> {
    let mut res: Vec<u8> = std::iter::repeat(0).take(8).collect();
    match context::get_endianess() {
        Big => BigEndian::write_u64(&mut res, v),
        Little => LittleEndian::write_u64(&mut res, v),
    };
    res
}

/// Unpacks a 1 byte `Vec` to a `u8`.
///
/// # Panics
///
/// Panics when `v.len() < 1`.
pub fn u8(v: &[u8]) -> u8 {
    v[0]
}

/// Unpacks a 2 byte `Vec` to a `u16`.
///
/// # Panics
///
/// Panics when `v.len() < 2`.
pub fn u16(v: &[u8]) -> u16 {
    match context::get_endianess() {
        Big => BigEndian::read_u16(v),
        Little => LittleEndian::read_u16(v),
    }
}

/// Unpacks a 4 byte `Vec` to a `u32`.
///
/// # Panics
///
/// Panics when `v.len() < 4`.
pub fn u32(v: &[u8]) -> u32 {
    match context::get_endianess() {
        Big => BigEndian::read_u32(v),
        Little => LittleEndian::read_u32(v),
    }
}

/// Unpacks an 8 byte `Vec` to a `u64`.
///
/// # Panics
///
/// Panics when `v.len() < 8`.
pub fn u64(v: &[u8]) -> u64 {
    match context::get_endianess() {
        Big => BigEndian::read_u64(v),
        Little => LittleEndian::read_u64(v),
    }
}

/// Automatically packs an integer in a [`context`] sensitive way.
/// Returns `None` if the passed integer is too large to be casted down.
pub fn pack<T>(v: T) -> Option<Vec<u8>>
where
    T: num_traits::ToPrimitive,
{
    match context::get_bits() {
        Bits::Eight => Some(p8(v.to_u8()?)),
        Bits::Sixteen => Some(p16(v.to_u16()?)),
        Bits::ThirtyTwo => Some(p32(v.to_u32()?)),
        Bits::SixtyFour => Some(p64(v.to_u64()?)),
    }
}

/// Automatically unpacks a buffer to an integer in a [`context`] sensitive way.
pub fn unpack(v: &[u8]) -> u64 {
    match context::get_bits() {
        Bits::Eight => u8(v) as u64,
        Bits::Sixteen => u16(v) as u64,
        Bits::ThirtyTwo => u32(v) as u64,
        Bits::SixtyFour => u64(v),
    }
}

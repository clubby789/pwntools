use crate::context::{
    self,
    Endianness::{Big, Little},
};
use crate::Bits;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::convert::TryInto;
use std::num::TryFromIntError;

/// Packs a `u8` to a 1-byte `Vec`
pub fn p8(v: u8) -> Vec<u8> {
    vec![v]
}

/// Packs a `u16` to a 2-byte `Vec`
pub fn p16(v: u16) -> Vec<u8> {
    let mut res = Vec::<u8>::new();
    match context::get_endianess() {
        Big => BigEndian::write_u16(&mut res, v),
        Little => LittleEndian::write_u16(&mut res, v),
    };
    res
}

/// Packs a `u32` to a 4-byte `Vec`
pub fn p32(v: u32) -> Vec<u8> {
    let mut res = Vec::<u8>::new();
    match context::get_endianess() {
        Big => BigEndian::write_u32(&mut res, v),
        Little => LittleEndian::write_u32(&mut res, v),
    };
    res
}

/// Packs a `u64` to an 8-byte `Vec`
pub fn p64(v: u64) -> Vec<u8> {
    let mut res = Vec::<u8>::new();
    match context::get_endianess() {
        Big => BigEndian::write_u64(&mut res, v),
        Little => LittleEndian::write_u64(&mut res, v),
    };
    res
}

/// Unpacks a 1 byte `Vec` to a `u8`
///
/// # Panics
///
/// Panics when `v.len() < 1`
pub fn u8(v: &[u8]) -> u8 {
    v[0]
}

/// Unpacks a 2 byte `Vec` to a `u16`
///
/// # Panics
///
/// Panics when `v.len() < 2`
pub fn u16(v: &[u8]) -> u16 {
    match context::get_endianess() {
        Big => BigEndian::read_u16(v),
        Little => LittleEndian::read_u16(v),
    }
}

/// Unpacks a 4 byte `Vec` to a `u32`
///
/// # Panics
///
/// Panics when `v.len() < 4`
pub fn u32(v: &[u8]) -> u32 {
    match context::get_endianess() {
        Big => BigEndian::read_u32(v),
        Little => LittleEndian::read_u32(v),
    }
}

/// Unpacks an 8 byte `Vec` to a `u64`
///
/// # Panics
///
/// Panics when `v.len() < 8`
pub fn u64(v: &[u8]) -> u64 {
    match context::get_endianess() {
        Big => BigEndian::read_u64(v),
        Little => LittleEndian::read_u64(v),
    }
}

/// Automatically packs an integer in a [`pwn::context`] sensitive way
/// Returns an error if the passed integer is too large to be casted down
pub fn pack<T>(v: T) -> Result<Vec<u8>, TryFromIntError>
where
    T: TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64>,
    TryFromIntError: From<<T as TryInto<u8>>::Error>,
    TryFromIntError: From<<T as TryInto<u16>>::Error>,
    TryFromIntError: From<<T as TryInto<u32>>::Error>,
    TryFromIntError: From<<T as TryInto<u64>>::Error>,
{
    match context::get_bits() {
        Bits::Eight => Ok(p8(v.try_into()?)),
        Bits::Sixteen => Ok(p16(v.try_into()?)),
        Bits::ThirtyTwo => Ok(p32(v.try_into()?)),
        Bits::SixtyFour => Ok(p64(v.try_into()?)),
    }
}

/// Automatically unpacks a buffer to an integer in a [`pwn::context`] sensitive way
pub fn unpack(v: &[u8]) -> u64 {
    match context::get_bits() {
        Bits::Eight => u8(v) as u64,
        Bits::Sixteen => u16(v) as u64,
        Bits::ThirtyTwo => u32(v) as u64,
        Bits::SixtyFour => u64(v)
    }
}

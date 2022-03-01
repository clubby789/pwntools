use crate::context::{
    context,
    Endianness::{Big, Little},
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::Cursor;

/// Packs a `u8` to a 1-byte `Vec`
pub fn p8(v: u8) -> Result<Vec<u8>, io::Error> {
    let mut res = Vec::<u8>::new();
    res.write_u8(v)?;
    Ok(res)
}

/// Packs a `u16` to a 2-byte `Vec`
pub fn p16(v: u16) -> Result<Vec<u8>, io::Error> {
    let mut res = Vec::<u8>::new();
    match context().endian {
        Big => res.write_u16::<BigEndian>(v)?,
        Little => res.write_u16::<LittleEndian>(v)?,
    };
    Ok(res)
}

/// Packs a `u32` to a 4-byte `Vec`
pub fn p32(v: u32) -> Result<Vec<u8>, io::Error> {
    let mut res = Vec::<u8>::new();
    match context().endian {
        Big => res.write_u32::<BigEndian>(v)?,
        Little => res.write_u32::<LittleEndian>(v)?,
    };
    Ok(res)
}

/// Packs a `u64` to an 8-byte `Vec`
pub fn p64(v: u64) -> Result<Vec<u8>, io::Error> {
    let mut res = Vec::<u8>::new();
    match context().endian {
        Big => res.write_u64::<BigEndian>(v)?,
        Little => res.write_u64::<LittleEndian>(v)?,
    };
    Ok(res)
}

/// Unpacks a 1 byte `Vec` to a `u8`
pub fn u8(v: Vec<u8>) -> Result<u8, io::Error> {
    let mut res = Cursor::new(v);
    res.read_u8()
}

/// Unpacks a 2 byte `Vec` to a `u16`
pub fn u16(v: Vec<u8>) -> Result<u16, io::Error> {
    let mut res = Cursor::new(v);
    Ok(match context().endian {
        Big => res.read_u16::<BigEndian>()?,
        Little => res.read_u16::<LittleEndian>()?,
    })
}

/// Unpacks a 4 byte `Vec` to a `u32`
pub fn u32(v: Vec<u8>) -> Result<u32, io::Error> {
    let mut res = Cursor::new(v);
    Ok(match context().endian {
        Big => res.read_u32::<BigEndian>()?,
        Little => res.read_u32::<LittleEndian>()?,
    })
}

/// Unpacks an 8 byte `Vec` to a `u64`
pub fn u64(v: Vec<u8>) -> Result<u64, io::Error> {
    let mut res = Cursor::new(v);
    Ok(match context().endian {
        Big => res.read_u64::<BigEndian>()?,
        Little => res.read_u64::<LittleEndian>()?,
    })
}

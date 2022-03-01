use pwn::context::{
    context_mut,
    Endianness::{Big, Little},
};
use pwn::util::packing::*;

/// Test the basic packing of unsigned integers
#[test]
pub fn test_packing() {
    context_mut().endian = Little;
    assert_eq!(p8(8).unwrap(), vec![8]);
    assert_eq!(p16(8).unwrap(), vec![8, 0]);
    assert_eq!(p32(8).unwrap(), vec![8, 0, 0, 0]);
    assert_eq!(p64(8).unwrap(), vec![8, 0, 0, 0, 0, 0, 0, 0]);
}

/// Test context-aware packing
#[test]
pub fn test_context_packing() {
    context_mut().endian = Big;
    assert_eq!(p8(8).unwrap(), vec![8]);
    assert_eq!(p16(8).unwrap(), vec![0, 8]);
    assert_eq!(p32(8).unwrap(), vec![0, 0, 0, 8]);
    assert_eq!(p64(8).unwrap(), vec![0, 0, 0, 0, 0, 0, 0, 8]);
}

/// Test the basic unpacking of unsigned integers
#[test]
pub fn test_unpacking() {
    context_mut().endian = Little;
    assert_eq!(u8(vec![8]).unwrap(), 8);
    assert_eq!(u16(vec![8, 0]).unwrap(), 8);
    assert_eq!(u32(vec![8, 0, 0, 0]).unwrap(), 8);
    assert_eq!(u64(vec![8, 0, 0, 0, 0, 0, 0, 0]).unwrap(), 8);
}

/// Test context-aware unpacking
#[test]
pub fn test_context_unpacking() {
    context_mut().endian = Big;
    assert_eq!(u8(vec![8]).unwrap(), 8);
    assert_eq!(u16(vec![0, 8]).unwrap(), 8);
    assert_eq!(u32(vec![0, 0, 0, 8]).unwrap(), 8);
    assert_eq!(u64(vec![0, 0, 0, 0, 0, 0, 0, 8]).unwrap(), 8);
}

use pwn::context::{
    self,
    Endianness::{Big, Little},
};
use pwn::util::packing::*;

#[test]
pub fn test_packing_little() {
    context::set_endianess(Little);
    assert_eq!(p8(8).unwrap(), vec![8]);
    assert_eq!(p16(8).unwrap(), vec![8, 0]);
    assert_eq!(p32(8).unwrap(), vec![8, 0, 0, 0]);
    assert_eq!(p64(8).unwrap(), vec![8, 0, 0, 0, 0, 0, 0, 0]);
}
#[test]
pub fn test_packing_big() {
    context::set_endianess(Big);
    assert_eq!(p8(8).unwrap(), vec![8]);
    assert_eq!(p16(8).unwrap(), vec![0, 8]);
    assert_eq!(p32(8).unwrap(), vec![0, 0, 0, 8]);
    assert_eq!(p64(8).unwrap(), vec![0, 0, 0, 0, 0, 0, 0, 8]);
}

#[test]
pub fn test_unpacking_little() {
    context::set_endianess(Little);
    assert_eq!(u8(vec![8]).unwrap(), 8);
    assert_eq!(u16(vec![8, 0]).unwrap(), 8);
    assert_eq!(u32(vec![8, 0, 0, 0]).unwrap(), 8);
    assert_eq!(u64(vec![8, 0, 0, 0, 0, 0, 0, 0]).unwrap(), 8);
}

#[test]
pub fn test_unpacking_big() {
    context::set_endianess(Big);
    assert_eq!(u8(vec![8]).unwrap(), 8);
    assert_eq!(u16(vec![0, 8]).unwrap(), 8);
    assert_eq!(u32(vec![0, 0, 0, 8]).unwrap(), 8);
    assert_eq!(u64(vec![0, 0, 0, 0, 0, 0, 0, 8]).unwrap(), 8);
}

#[test]
pub fn test_multi_threading() {
    use std::thread;
    // Ensure that thread-local contexts work
    let handles = [test_packing_little, test_packing_big, test_unpacking_little, test_unpacking_big].map(|f| {
        thread::spawn(move || for _ in 0..100 {f()})
    });
    for h in handles {
        h.join().unwrap();
    }
}
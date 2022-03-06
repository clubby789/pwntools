use pwn::context::Bits::SixtyFour;
use pwn::context::Endianness::Little;
use pwn::context::{self, AMD64, I386};

/// Test `Context`
#[test]
fn test_context() {
    assert_eq!(context::get_arch(), I386);
    assert_eq!(context::get_arch(), I386);
    context::set_arch(AMD64);
    assert_eq!(context::get_arch(), AMD64);
    assert_eq!(context::get_endianess(), Little);
    assert_eq!(context::get_bits(), SixtyFour);
}

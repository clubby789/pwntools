use pwn::context::Bits::SixtyFour;
use pwn::context::Endianness::Little;
use pwn::context::{context, context_mut, AMD64, I386};

/// Test `Context`
#[test]
fn test_context() {
    assert_eq!(context().arch, I386);
    context_mut().set_arch(AMD64);
    assert_eq!(context().arch, AMD64);
    assert_eq!(context().endian, Little);
    assert_eq!(context().bits, SixtyFour);
}

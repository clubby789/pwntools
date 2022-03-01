use pwn::context::Bits::SixtyFour;
use pwn::context::Endianness::Little;
use pwn::context::{context_mut, AMD64, I386};

/// Test `Context`
#[test]
fn test_context() {
    let mut ctx = context_mut();
    assert_eq!(ctx.arch, I386);
    assert_eq!(ctx.arch, I386);
    ctx.set_arch(AMD64);
    assert_eq!(ctx.arch, AMD64);
    assert_eq!(ctx.endian, Little);
    assert_eq!(ctx.bits, SixtyFour);
}

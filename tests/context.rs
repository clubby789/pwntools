use pwntools::context::{context, I386, AMD64};

/// Test `Context`
#[test]
fn test_context() {
    assert_eq!(context().arch, I386);
    context().arch = AMD64;
    assert_eq!(context().arch, AMD64);
}
use pwn::{context, flat, Flatten, I386};

#[test]
fn test_flatten() {
    context::set_arch(I386);
    assert_eq!(
        flat(&[1u32, 2, 3]),
        b"\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00"
    );
    let dict: Vec<(usize, Box<dyn Flatten>)> =
        vec![(12, Box::new(0x41414141u32)), (24, Box::new(b"Hello!\xff"))];
    assert_eq!(flat(&*dict), b"aaaaaaaaaaaaAAAAaaaaaaaaHello!\xff");
}

#[test]
#[should_panic]
fn test_overlap() {
    context::set_arch(I386);
    // Suppress backtrace during tests
    use std::panic::PanicInfo;
    let f = |_: &PanicInfo| {};
    std::panic::set_hook(Box::new(f));
    flat(&*vec![
        (0usize, Box::new(0x41414141u32) as Box<dyn Flatten>),
        (2, Box::new(b"Overlap") as _),
    ]);
}

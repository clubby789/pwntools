use crate::{pack};
use duplicate::duplicate_item;

/// Flatten an item into bytes in a [`crate::context`]-aware way.
/// Un-filled bytes in the buffer will be replaced with `'a'`.
/// # Examples
/// Packing a simple list of numbers
/// ```
/// use pwn::{flat, context, I386};
/// context::set_arch(I386);
/// assert_eq!(flat(&[1u32, 2, 3]), b"\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00")
/// ```
/// Pack a complex sequence of items with offsets
/// ```
/// use pwn::{flat, context, I386, Flatten};
/// context::set_arch(I386);
/// let mapping: Vec<(usize, Box<dyn Flatten>)> = vec![
///     (12, Box::new(0x41414141u32)),
///     (24, Box::new(b"Hello!\xff"))
/// ];
///assert_eq!(flat(&*mapping), b"aaaaaaaaaaaaAAAAaaaaaaaaHello!\xff");
/// ```
/// # Warnings
/// * Sequences of `u8` will *not* be packed according to the context, and instead returned as passed.
/// This is to allow byte-strings to be included in `flat()`.
/// * Overlapping values will result in a panic.
pub fn flat(item: impl Flatten) -> Vec<u8> {
    item.flatten()
}

/// An object that can be 'flattened' down to a byte buffer.
pub trait Flatten {
    /// Return the 'flattened' form of `self`.
    fn flatten(&self) -> Vec<u8>;
}
#[duplicate_item(
    int_type;
    [u16];
    [u32];
    [u64];
)]
impl Flatten for int_type {
    fn flatten(&self) -> Vec<u8> {
        pack(*self).expect("Value was too large to be flattened")
    }
}

impl Flatten for &str {
    fn flatten(&self) -> Vec<u8> {
        self.bytes().collect()
    }
}

// Special case - already bytes, don't pack
impl Flatten for u8 {
    fn flatten(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl<T> Flatten for &[T]
where
    T: Flatten,
{
    fn flatten(&self) -> Vec<u8> {
        self.iter()
            .map(|v| v.flatten())
            .collect::<Vec<Vec<_>>>()
            .concat()
    }
}

impl<T, const N: usize> Flatten for &[T; N]
where
    T: Flatten,
{
    fn flatten(&self) -> Vec<u8> {
        self.as_slice().flatten()
    }
}

impl<I> Flatten for &[(I, Box<dyn Flatten>)]
where
    usize: From<I>,
    I: Copy,
{
    fn flatten(&self) -> Vec<u8> {
        let mut map: Vec<(usize, &Box<dyn Flatten>)> =
            self.iter().map(|(i, f)| ((*i).into(), f)).collect();
        map.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        // Prepare a vec with near the right capacity
        let mut out: Vec<Option<u8>> = Vec::with_capacity(map.first().map_or(0, |(i, _)| *i));
        for (i, val) in map {
            let flattened = val.flatten();

            let end_i = i + flattened.len();
            if end_i > out.len() {
                out.extend(std::iter::repeat(None).take(end_i - out.len()));
            }
            // If any of the values spliced out are `Some`, we're overlapping
            if out
                .splice(
                    i..i + flattened.len(),
                    flattened.into_iter().map(Some).collect::<Vec<Option<u8>>>(),
                )
                .any(|v: Option<u8>| v.is_some())
            {
                panic!("Values in flat() overlap");
            }
        }
        out.iter().map(|v| v.unwrap_or(b'a')).collect()
    }
}

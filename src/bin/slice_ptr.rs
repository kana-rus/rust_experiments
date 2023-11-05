use std::slice;

fn impl_asref_into_raw_parts(data: impl AsRef<[u8]>) -> (*const u8, usize) {
    let slice = data.as_ref();
    (slice.as_ptr(), slice.len())
}

fn main() {
    let (ptr, len) = impl_asref_into_raw_parts(b"Hello, world!");
    assert_eq!(unsafe {slice::from_raw_parts(ptr, len)}, b"Hello, world!");
    let (ptr, len) = impl_asref_into_raw_parts(b"Hello, world!".to_vec());
    assert_eq!(unsafe {slice::from_raw_parts(ptr, len)}, b"Hello, world!");
}

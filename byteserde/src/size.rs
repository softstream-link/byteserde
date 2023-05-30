/// Trait type used in advanced cases when serializing and deserializing an optional block of a byte
/// stream whose type is represented by a `struct` whose size is deterministict at compile time.
/// 
/// Typically will be implemented using a `byteserde_derive::ByteSerializedSizeOf` proc macro.
/// 
/// # Guarantees
/// * Must return the number of bytes the implementing structure will occupy in a byte stream 
///  when serialized. This is instance `independent` trait and may not be possible to implement for 
/// `struct`s whose elements might be allocated on the heap at run time, example String, Vec, etc.
pub trait ByteSerializedSizeOf{
    fn byte_size() -> usize;
}
macro_rules! size_of {
    ($t:ty) => {
        impl ByteSerializedSizeOf for $t {
            fn byte_size() -> usize {
                std::mem::size_of::<$t>()
            }
        }
    };
}
size_of!(u8);
size_of!(i8);
size_of!(u16);
size_of!(i16);
size_of!(u32);
size_of!(i32);
size_of!(u64);
size_of!(i64);
size_of!(u128);
size_of!(i128);
size_of!(usize);
size_of!(isize);

/// Trait type used in advanced cases when serializing and deserializing an optional block of a byte
/// stream whose type is represented by a `struct` whose size is `NOT` deterministict at compile time.
/// 
/// Typically will be implemented using a `byteserde_derive::ByteSerializedLenOf` proc macro.
/// 
/// # Guarantees
/// * Must return the number of bytes a specific `instance` of implementing structure will occupy in a byte stream
/// when serialized. This is instance `dependent` trait and might return a differet length for each instance,
/// example String, Vec, etc.
pub trait ByteSerializedLenOf{
    fn byte_len(&self) -> usize;
}

impl ByteSerializedLenOf for String {
    fn byte_len(&self) -> usize {
        self.len()
    }
}
impl ByteSerializedLenOf for char {
    fn byte_len(&self) -> usize {
        self.len_utf8()
    }
}
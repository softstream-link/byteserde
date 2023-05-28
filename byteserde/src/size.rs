pub trait ByteSerializedSizeOf{
    fn byte_size() -> usize;
}

pub trait ByteSerializedLenOf{
    fn byte_len(&self) -> usize;
}
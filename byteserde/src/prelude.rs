pub use super::des_slice::{ByteDeserializeSlice, ByteDeserializerSlice, from_serializer_stack, from_serializer_heap, from_slice};
pub use super::des_bytes::{ByteDeserializeBytes, ByteDeserializerBytes, from_bytes};
// pub use super::error::Result;
pub use super::error::SerDesError;
pub use super::ser_heap::{to_bytes_heap, to_serializer_heap};
pub use super::ser_heap::{ByteSerializeHeap, ByteSerializerHeap};
pub use super::ser_stack::{to_bytes_stack, to_serializer_stack};
pub use super::ser_stack::{ByteSerializeStack, ByteSerializerStack};
pub use super::size::{ByteSerializedLenOf, ByteSerializedSizeOf};
use bytes::{Bytes, BytesMut};

use crate::utils::{
    hex::{to_hex_line, to_hex_pretty},
    numerics::{be_bytes::ToBeBytes, le_bytes::ToLeBytes, ne_bytes::ToNeBytes},
};

use std::{
    any::type_name,
    fmt::{Debug, LowerHex},
};
/// Trait type accepted by [ByteSerializerHeap] for serialization.
/// Example: Define structure and manually implement ByteSerializeHeap trait then use it to serialize.
/// ```
/// use ::byteserde::prelude::*;
/// struct MyStruct { a: u8, }
/// impl ByteSerializeHeap for MyStruct {
///    fn byte_serialize_heap(&self, ser: &mut ByteSerializerHeap) -> byteserde::error::Result<()> {
///       ser.serialize_bytes_slice(&[self.a])?;
///      Ok(())
///   }
/// }
///
/// let s = MyStruct { a: 0x01 };
///
/// let ser: ByteSerializerHeap = to_serializer_heap(&s).unwrap();
/// assert_eq!(ser.len(), 1);
///
/// let bytes = to_bytes_heap::<MyStruct>(&s).unwrap();
/// assert_eq!(bytes.len(), 1);
/// ```
pub trait ByteSerializeHeap {
    fn byte_serialize_heap(&self, ser: &mut ByteSerializerHeap) -> crate::error::Result<()>;
}
/// A byte buffer allocated on heap backed by `Vec<u8>`, can be reused and recycled by calling [Self::clear()].
/// Example: Create a Buffer and serialize data into it.
/// ```
/// use ::byteserde::prelude::*;
/// let mut ser = ByteSerializerHeap::default();
///
/// ser.serialize_bytes_slice(&[0x01]).unwrap();
///
/// assert_eq!(ser.len(), 1);
/// assert_eq!(ser.is_empty(), false);
///
/// ser.clear();
/// assert_eq!(ser.is_empty(), true);
#[derive(Debug, Clone, Default)]
pub struct ByteSerializerHeap {
    bytes: BytesMut,
}
impl ByteSerializerHeap {
    pub fn with_capacity(cap: usize) -> Self {
        ByteSerializerHeap {
            bytes: BytesMut::with_capacity(cap),
        }
    }
}
/// Provides a convenient way to view buffer content as both HEX and ASCII bytes where printable.
/// supports both forms of alternate formatting `{:x}` and `{:#x}`.
/// ```
/// use ::byteserde::prelude::*;
/// let mut ser = ByteSerializerHeap::default();
/// ser.serialize_bytes_slice(&[0x01, 0x02, 0x03, 0x04, 0x05]);
/// println!("{:x}", ser);
/// println!("{:#x}", ser);
/// ```
impl LowerHex for ByteSerializerHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = match f.alternate() {
            true => format!("\n{hex}", hex = to_hex_pretty(self.as_slice())),
            false => to_hex_line(&self.bytes),
        };
        let len = self.len();
        let name = type_name::<Self>().split("::").last().unwrap();
        write!(f, "{name} {{ len: {len}, bytes: {bytes} }}")
    }
}
impl ByteSerializerHeap {
    /// Clears underlying Vec with out affecting its capacity.
    #[inline]
    pub fn clear(&mut self) {
        self.bytes.clear();
    }
    /// Returns the length of the buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    /// Returns true if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.len() == 0
    }
    /// Current allocated capacity, can grow as needed.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }
    /// Returns the number of bytes available for writing without need for reallocation
    #[inline]
    pub fn available(&self) -> usize {
        self.bytes.capacity() - self.bytes.len()
    }
    /// Returns entire content of the buffer as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[0..]
    }
    /// Writes a slice of bytes into the buffer.
    pub fn serialize_bytes_slice(&mut self, bytes: &[u8]) -> crate::error::Result<&mut Self> {
        self.bytes.extend_from_slice(bytes);
        Ok(self)
    }
    /// This is a convenience method to serialize all rust's numeric primitives into the buffer using `native` endianess.
    /// ToNeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::*;
    /// let mut ser = ByteSerializerHeap::default();
    /// ser.serialize_ne(0x1_u16);
    /// ser.serialize_ne(0x1_i16);
    /// // ... etc
    /// ```
    pub fn serialize_ne<const N: usize, T: ToNeBytes<N>>(
        &mut self,
        v: T,
    ) -> crate::error::Result<&mut Self> {
        self.serialize_bytes_slice(&v.to_bytes())
    }
    /// This is a convenience method to serialize all rust's numeric primitives into the buffer using `little` endianess.
    /// ToLeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::*;
    /// let mut ser = ByteSerializerHeap::default();
    /// ser.serialize_le(0x1_u16);
    /// ser.serialize_le(0x2_i16);
    /// println!("{:x}", ser);
    /// assert_eq!(ser.len(), 4);
    /// ```
    pub fn serialize_le<const N: usize, T: ToLeBytes<N>>(
        &mut self,
        v: T,
    ) -> crate::error::Result<&mut Self> {
        self.serialize_bytes_slice(&v.to_bytes())
    }
    /// This is a convenience method to serialize all rust's numeric primitives into the buffer using `big` endianess.
    /// ToBeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::*;
    /// let mut ser = ByteSerializerHeap::default();
    /// ser.serialize_be(0x1_u16);
    /// ser.serialize_be(0x2_i16);
    /// println!("{:x}", ser);
    /// assert_eq!(ser.len(), 4);
    /// ```
    pub fn serialize_be<const N: usize, T: ToBeBytes<N>>(
        &mut self,
        v: T,
    ) -> crate::error::Result<&mut Self> {
        self.serialize_bytes_slice(&v.to_bytes())
    }

    pub fn serialize<T: ByteSerializeHeap>(&mut self, v: &T) -> crate::error::Result<&mut Self> {
        v.byte_serialize_heap(self)?;
        Ok(self)
    }
}
/// Analogous to [to_bytes_heap] but returns an instance of [ByteSerializerHeap]
pub fn to_serializer_heap<T>(v: &T) -> crate::error::Result<ByteSerializerHeap>
where
    T: ByteSerializeHeap,
{
    let mut ser = ByteSerializerHeap::default();
    v.byte_serialize_heap(&mut ser)?;
    Result::Ok(ser)
}
/// Analogous to [to_serializer_heap] but returns an instance of [`Vec<u8>`]
pub fn to_bytes_heap<T>(v: &T) -> crate::error::Result<Bytes>
where
    T: ByteSerializeHeap,
{
    let ser = to_serializer_heap(v)?;
    Ok(ser.bytes.freeze())
}

use crate::{
    error::{Result, SerDesError},
    utils::{
        hex::{to_hex_line, to_hex_pretty},
        numerics::{be_bytes::ToBeBytes, le_bytes::ToLeBytes, ne_bytes::ToNeBytes},
    },
};

use std::{
    any::type_name,
    fmt::{Debug, LowerHex},
};

/// Trait type accepted by [ByteSerializerStack] for serialization.
/// Example: Define structure and manually implement ByteSerializeStack trait then use it to serialize.
/// ```
/// use ::byteserde::prelude::*;
///
/// struct MyStruct { a: u8, }
/// impl ByteSerializeStack for MyStruct {
///     fn byte_serialize_stack<const CAP: usize>(&self, ser: &mut ByteSerializerStack<CAP>) -> Result<()> {
///         ser.serialize_bytes_slice(&[self.a])?;
///         Ok(())
///    }
/// }
///
/// let s = MyStruct { a: 0x01 };
///
/// let ser: ByteSerializerStack<128> = to_serializer_stack(&s).unwrap();
/// assert_eq!(ser.len(), 1);
///
/// let bytes = to_bytes_stack::<128, MyStruct>(&s).unwrap();
/// assert_eq!(bytes.len(), 128);
/// ```
pub trait ByteSerializeStack {
    fn byte_serialize_stack<const CAP: usize>(
        &self,
        ser: &mut ByteSerializerStack<CAP>,
    ) -> Result<()>;
}
/// A byte buffer allocated on stack backed by `[u8; CAP]`, can be reused and recyled by calling [Self::reset()].
/// Example: Creates a buffer with 128 bytes capacity and serializes data into it.
/// ```
/// use ::byteserde::prelude::*;
/// let mut ser = ByteSerializerStack::<128>::default();
/// assert_eq!(ser.is_empty(), true);
///
/// ser.serialize_bytes_slice(&[0x01]);
///
/// assert_eq!(ser.capacity(), 128);
/// assert_eq!(ser.avail(), 128 - 1);
/// assert_eq!(ser.len(), 1);
/// assert_eq!(ser.is_empty(), false);
///
/// ser.reset();
/// assert_eq!(ser.is_empty(), true);
/// ```
#[derive(Debug, Clone)]
pub struct ByteSerializerStack<const CAP: usize> {
    bytes: [u8; CAP],
    len: usize,
}
/// Provides a conveninet way to view buffer content as both HEX and ASCII bytes where printable.
/// supports both forms of alternate formatting `{:x}` and `{:#x}`.
/// ```
/// use ::byteserde::prelude::*;
/// let mut ser = ByteSerializerStack::<128>::default();
/// println ! ("{:#x}", ser); // upto 16 bytes per line
/// println ! ("{:x}", ser);  // single line
/// ```
impl<const CAP: usize> LowerHex for ByteSerializerStack<CAP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = match f.alternate() {
            true => format!("\n{hex}", hex = to_hex_pretty(self.bytes())),
            false => to_hex_line(self.bytes()),
        };
        let len = self.len;
        let name = type_name::<Self>().split("::").last().unwrap();
        write!(f, "{name} {{ len: {len}, cap: {CAP}, bytes: {bytes} }}")
    }
}
impl<const CAP: usize> Default for ByteSerializerStack<CAP> {
    fn default() -> Self {
        ByteSerializerStack {
            // TODO this causes twice amount of writes first to set tozero then to write value hence serialize takes 2x deserialize, need to figure out how to start uninitialized array
            // let mut arr: [u16; 5];
            // unsafe {
            //     let mut raw_arr: [mem::MaybeUninit<u16>; 5] = mem::MaybeUninit::uninit().assume_init();
            //     for (i, elem) in raw_arr.iter_mut().enumerate() {
            //         // Initialize each element of the array using the dereference operator and the write() method
            //         elem.as_mut_ptr().write(i as u16);
            //     }
            //     // Convert the raw array to a regular array using the transmute() method
            //     arr = mem::transmute::<[mem::MaybeUninit<u16>; 5], [u16; 5]>(raw_arr);
            // }
            bytes: [0x00_u8; CAP],
            len: 0,
        }
    }
}
impl<const CAP: usize> ByteSerializerStack<CAP> {
    /// Resets the buffer to zero length, does not clear the buffer. Next serialize will write from start of buffer.
    pub fn reset(&mut self) {
        self.len = 0;
    }
    /// Returns the length of the buffer, number of bytes written.
    pub fn len(&self) -> usize {
        self.len
    }
    /// Returns true if [Self::len()] is zero.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Returns const capacity of the buffer.
    pub fn capacity(&self) -> usize {
        CAP
    }
    /// Returns number of available slots in the beffer. [Self::capacity()] - [Self::len()]
    pub fn avail(&self) -> usize {
        CAP - self.len
    }
    /// Returns a slice of the buffer containing the bytes written, less any unused slots.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[0..self.len]
    }

    /// Serializes entire slice into the buffer, returns [SerDesError] if required capacity is exceeded.
    pub fn serialize_bytes_slice(&mut self, bytes: &[u8]) -> Result<&mut Self> {
        let input_len = bytes.len();
        let avail = self.avail();
        match input_len > avail {
            false => {
                // TODO try using copy from slice as safer alternative
                // T.serialize(&serializer) - reuse serializer each iter
                //         time:   [62.164 ns 64.446 ns 67.565 ns]
                // self.bytes[self.len..self.len+input_len].copy_from_slice(&bytes);
                // 50% improvement using unsafe
                // T.serialize(&serializer) - reuse serializer each iter
                //         time:   [29.611 ns 30.166 ns 30.881 ns]
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        bytes.as_ptr(),
                        self.bytes.as_mut_ptr().add(self.len),
                        bytes.len(),
                    );
                }

                self.len += bytes.len();
                Result::Ok(self)
            }
            true => {
                // panic!("blah"); // TODO format kills performance
                Result::Err(SerDesError {
                    message: format!(
                        "adding {input_len} bytes, {avail} slots available. input: {bytes:?} buffer: {self:x}",
                    ),
                })
            }
        }
    }
    /// This is a convenience method to serialize all rust's numeric primitives into the buffer using `native` endianess.
    /// ToNeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::*;
    /// let mut ser = ByteSerializerStack::<128>::default();
    /// ser.serialize_ne(0x1_u16);
    /// ser.serialize_ne(0x1_i16);
    /// // ... etc
    /// ```
    pub fn serialize_ne<const N: usize, T: ToNeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes_slice(&v.to_bytes())
    }
    /// This is a convenience method to serialize all rust's numeric primitives into the buffer using `little` endianess.
    /// ToLeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::*;
    /// let mut ser = ByteSerializerStack::<128>::default();
    /// ser.serialize_le(0x1_u16);
    /// ser.serialize_le(0x1_i16);
    /// // ... etc
    /// ```
    pub fn serialize_le<const N: usize, T: ToLeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes_slice(&v.to_bytes())
    }
    /// This is a convenience method to serialize all rust's numeric primitives into the buffer using `big` endianess.
    /// ToBeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::*;
    /// let mut ser = ByteSerializerStack::<128>::default();
    /// ser.serialize_be(0x1_u16);
    /// ser.serialize_be(0x1_i16);
    /// // ... etc
    /// ```
    pub fn serialize_be<const N: usize, T: ToBeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes_slice(&v.to_bytes())
    }
    /// Serializes a `struct` that implements [ByteSerializeStack] trait into the buffer.
    pub fn serialize<T: ByteSerializeStack>(&mut self, v: &T) -> Result<&mut Self> {
        v.byte_serialize_stack(self)?;
        Ok(self)
    }
}

/// Analogous to [`to_bytes_stack::<CAP>()`], but returns an instance of [`ByteSerializerStack<CAP>`].
pub fn to_serializer_stack<const CAP: usize, T>(v: &T) -> Result<ByteSerializerStack<CAP>>
where
    T: ByteSerializeStack,
{
    let mut ser = ByteSerializerStack::<CAP>::default();
    v.byte_serialize_stack(&mut ser)?;
    Result::Ok(ser)
}
/// Analogous to [`to_serializer_stack::<CAP>()`], but returns just the array of bytes `[u8; CAP]`.
/// Note that this is not a `&[u8]` slice, but an array of bytes with length CAP even if
/// the actual length of the serialized data is less.
pub fn to_bytes_stack<const CAP: usize, T>(v: &T) -> Result<[u8; CAP]>
where
    T: ByteSerializeStack,
{
    let ser = to_serializer_stack(v)?;
    Ok(ser.bytes)
}

/////////////////////

/// Trait type accepted by [ByteSerializerHeap] for serialization.
/// Example: Define structure and manually implement ByteSerializeHeap trait then use it to serialize.
/// ```
/// use ::byteserde::prelude::*;
/// struct MyStruct { a: u8, }
/// impl ByteSerializeHeap for MyStruct {
///    fn byte_serialize_heap(&self, ser: &mut ByteSerializerHeap) -> Result<()> {
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
    fn byte_serialize_heap(&self, ser: &mut ByteSerializerHeap) -> Result<()>;
}
/// A byte buffer allocated on heap backed by `Vec<u8>`, can be reused and recycled by calling [Self::reset()].
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
/// ser.reset();
/// assert_eq!(ser.is_empty(), true);
#[derive(Debug, Default, Clone)]
pub struct ByteSerializerHeap {
    bytes: Vec<u8>,
}
/// Provides a convenient way to view buffer content as both HEX and ASCII bytes where prinable.
/// supports both forms of alternate formatting `{:x}` and `{:#x}`.
/// ```
/// use ::byteserde::prelude::*;
/// let mut ser = ByteSerializerHeap::default();
/// println!("{:x}", ser);
/// println!("{:#x}", ser);
/// ```
impl LowerHex for ByteSerializerHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = match f.alternate() {
            true => format!("\n{hex}", hex = to_hex_pretty(self.bytes())),
            false => to_hex_line(&self.bytes),
        };
        let len = self.len();
        let name = type_name::<Self>().split("::").last().unwrap();
        write!(f, "{name} {{ len: {len}, bytes: {bytes} }}")
    }
}
impl ByteSerializerHeap {
    /// Clears underlying Vec with out affecting its capacity.
    pub fn reset(&mut self) {
        self.bytes.clear();
    }
    /// Returns the length of the buffer.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.len() == 0
    }
    /// Current allocated capacity, can grow as needed.
    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }
    /// Returns the number of bytes available for writing without need for realocation
    pub fn available(&self) -> usize {
        self.bytes.capacity() - self.bytes.len()
    }
    /// Returns entire content of the buffer as a slice.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[0..]
    }
    /// Writes a slice of bytes into the buffer.
    pub fn serialize_bytes_slice(&mut self, bytes: &[u8]) -> Result<&mut Self> {
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
    pub fn serialize_ne<const N: usize, T: ToNeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes_slice(&v.to_bytes())
    }
    /// This is a convenience method to serialize all rust's numeric primitives into the buffer using `little` endianess.
    /// ToLeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::*;
    /// let mut ser = ByteSerializerHeap::default();
    /// ser.serialize_le(0x1_u16);
    /// ser.serialize_le(0x1_i16);
    /// ```
    pub fn serialize_le<const N: usize, T: ToLeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes_slice(&v.to_bytes())
    }
    /// This is a convenience method to serialize all rust's numeric primitives into the buffer using `big` endianess.
    /// ToBeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::*;
    /// let mut ser = ByteSerializerHeap::default();
    /// ser.serialize_be(0x1_u16);
    /// ser.serialize_be(0x1_i16);
    /// ```
    pub fn serialize_be<const N: usize, T: ToBeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes_slice(&v.to_bytes())
    }

    pub fn serialize<T: ByteSerializeHeap>(&mut self, v: &T) -> Result<&mut Self> {
        v.byte_serialize_heap(self)?;
        Ok(self)
    }
}
/// Analogous to [to_bytes_heap] but returns an instance of [ByteSerializerHeap]
pub fn to_serializer_heap<T>(v: &T) -> Result<ByteSerializerHeap>
where
    T: ByteSerializeHeap,
{
    let mut ser = ByteSerializerHeap::default();
    v.byte_serialize_heap(&mut ser)?;
    Result::Ok(ser)
}
/// Analogous to [to_serializer_heap] but returns an instance of [`Vec<u8>`]
pub fn to_bytes_heap<T>(v: &T) -> Result<Vec<u8>>
where
    T: ByteSerializeHeap,
{
    let ser = to_serializer_heap(v)?;
    Ok(ser.bytes)
}

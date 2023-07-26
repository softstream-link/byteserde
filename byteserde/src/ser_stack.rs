use bytes::Bytes;

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
/// let (bytes, len) = to_bytes_stack::<128, MyStruct>(&s).unwrap();
/// assert_eq!(bytes.len(), 128);
/// assert_eq!(len, 1);
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
/// ser.clear();
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
/// ser.serialize_bytes_slice(&[0x01, 0x02, 0x03, 0x04, 0x05]);
/// println ! ("{:#x}", ser); // upto 16 bytes per line
/// println ! ("{:x}", ser);  // single line
/// ```
impl<const CAP: usize> LowerHex for ByteSerializerStack<CAP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = match f.alternate() {
            true => format!("\n{hex}", hex = to_hex_pretty(self.as_slice())),
            false => to_hex_line(self.as_slice()),
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
    pub fn clear(&mut self) {
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
    /// Returns number of available slots in the buffer. [Self::capacity()] - [Self::len()]
    pub fn avail(&self) -> usize {
        CAP - self.len
    }
    /// Returns a slice of the buffer containing the bytes written, less any unused slots.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[0..self.len]
    }

    /// Serializes entire slice into the buffer, returns [SerDesError] if required capacity is exceeded.
    // #[inline]
    pub fn serialize_bytes_slice(&mut self, bytes: &[u8]) -> Result<&mut Self> {
        let input_len = bytes.len();
        let avail = self.avail();
        match input_len > avail {
            false => {
                // safe -> self.bytes[self.len..self.len+bytes.len()].copy_from_slice(bytes);
                // safe 60ns vs 15ns unsafe using bench and reference struct
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
            true => Err(self.error(bytes.len())),
        }
    }

    fn error(&self, n: usize) -> SerDesError {
        SerDesError {
            message: format!("Failed to add a slice size: {n} into {self:x}",),
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
// #[inline] - TODO - panics during benchmarking
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
#[inline(always)]
pub fn to_bytes_stack<const CAP: usize, T>(v: &T) -> Result<([u8; CAP], usize)>
// pub fn to_bytes_stack<const CAP: usize, T>(v: &T) -> Result<([u8; CAP], usize)>
where
    T: ByteSerializeStack,
{
    let ser = to_serializer_stack(v)?;
    Ok((ser.bytes, ser.len()))
}

impl ByteSerializeStack for Bytes {
    fn byte_serialize_stack<const CAP: usize>(
        &self,
        ser: &mut ByteSerializerStack<CAP>,
    ) -> Result<()> {
        ser.serialize_bytes_slice(&self[..])?;
        Ok(())
    }
}
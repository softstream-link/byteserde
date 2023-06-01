use std::fmt::{Debug, LowerHex};

use crate::{
    error::{Result, SerDesError},
    prelude::ByteSerializerHeap,
    utils::{
        hex::{to_hex_line, to_hex_pretty},
        numerics::{be_bytes::FromBeBytes, le_bytes::FromLeBytes, ne_bytes::FromNeBytes},
    },
};

use super::ser::ByteSerializerStack;

/// Utility struct with a number of methods to enable deserialization of bytes into various types
/// ```
/// use ::byteserde::prelude::ByteDeserializer;
/// let bytes = &[0x01, 0x00, 0x02, 0x00, 0x00, 0x03];
/// let mut des = ByteDeserializer::new(bytes);
/// assert_eq!(des.remaining(), 6);
/// assert_eq!(des.idx(), 0);
/// assert_eq!(des.len(), 6);
///
/// let first: u8 = des.deserialize_bytes_slice(1).unwrap()[0];
/// assert_eq!(first , 1);
///
/// let second: [u8; 2] = des.deserialize_bytes_array().unwrap();
/// assert_eq!(second, [0x00, 0x02]);
///
/// let remaining: &[u8] = des.deserialize_bytes_slice_remaining();
/// assert_eq!(remaining, &[0x00, 0x00, 0x03]);
/// ```
#[derive(Debug)]
pub struct ByteDeserializer<'x> {
    bytes: &'x [u8],
    idx: usize,
}

/// Provides a conveninet way to view buffer content as both HEX and ASCII bytes where printable.
/// supports both forms of alternate
/// ```
/// use byteserde::des::ByteDeserializer;
/// let mut des = ByteDeserializer::new(&[0x01, 0x00, 0x02, 0x00, 0x00, 0x03]);
/// println ! ("{:#x}", des); // upto 16 bytes per line
/// println ! ("{:x}", des);  // single line
/// ```
impl<'x> LowerHex for ByteDeserializer<'x> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = match f.alternate() {
            true => format!("\n{hex}", hex = to_hex_pretty(self.bytes)),
            false => to_hex_line(self.bytes),
        };
        let len = self.bytes.len();
        let idx = self.idx;
        write!(
            f,
            "ByteDeserializer {{ len: {len}, idx: {idx}, bytes: {bytes} }}",
        )
    }
}

impl<'x> ByteDeserializer<'x> {
    pub fn new(bytes: &[u8]) -> ByteDeserializer {
        ByteDeserializer { bytes, idx: 0 }
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }

    /// Tracks the bytes read and always set to the next unread byte in the buffer. This is an inverse of [Self::remaining()]
    pub fn idx(&self) -> usize {
        self.idx
    }
    /// Number of bytes remaining to be deserialized, this is an inverse of [Self::idx()]
    pub fn remaining(&self) -> usize {
        self.len() - self.idx
    }

    // Number of bytes in the buffer does not change as deserialization progresses unlike [Self::remaining()] and [Self::idx()]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }
    /// returns an array of bytes whose length will be equal to argument `N`
    pub fn deserialize_bytes_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let a = self.peek_bytes_array()?;
        self.idx += N;
        Ok(a)
    }

    pub fn peek_bytes_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        // DON"T call .._slice as it halfs perforamnce let slice = self.deserialize_slice(N)?;
        match self.bytes.get(self.idx..self.idx + N) {
            Some(v) => Ok(v.try_into().expect("Failed to convert &[u8] into [u8; N]")),
            None => Err(self.error(N)),
        }
    }

    #[cold]
    fn error(&mut self, n: usize) -> SerDesError {
        // moving error into a fn improves performance by 10%
        // from_bytes - reuse ByteDeserializer
        // time:   [39.251 ns 39.333 ns 39.465 ns]
        // change: [-12.507% -11.603% -10.612%] (p = 0.00 < 0.05)
        // Performance has improved.
        SerDesError {
            message: format!("Failed to get a slice size: {n} bytes from {self:x}"),
        }
    }
    /// consumes all of the remaining bytes in the buffer and returns them as slice
    pub fn deserialize_bytes_slice_remaining(&mut self) -> &[u8] {
        let slice = &self.bytes[self.idx..];
        self.idx += slice.len();
        slice
    }
    /// consumes `len` bytes from the buffer and returns them as slice if successful.
    /// Fails if `len` is greater then [Self::remaining()]
    pub fn deserialize_bytes_slice(&mut self, len: usize) -> Result<&[u8]> {
        match self.bytes.get(self.idx..self.idx + len) {
            Some(v) => {
                self.idx += len;
                Ok(v)
            }
            None => Err(self.error(len)),
        }
    }

    #[inline(always)]
    pub fn deserialize_u8(&mut self) -> Result<u8> {
        let res = self.bytes.first();
        match res {
            Some(v) => {
                self.idx += 1;
                Ok(*v)
            }
            None => Err(self.error(1)),
        }
    }
    #[inline(always)]
    pub fn deserialize_i8(&mut self) -> Result<i8> {
        let res = self.bytes.first();
        match res {
            Some(v) => {
                self.idx += 1;
                Ok(*v as i8)
            }
            None => Err(self.error(1)),
        }
    }
    /// moves the index forward by `len` bytes, intended to be used in combination with [Self::peek_bytes_slice()]
    pub fn advance_idx(&mut self, len: usize) {
        self.idx += len;
    }
    /// produces with out consuming `len` bytes from the buffer and returns them as slice if successful.
    pub fn peek_bytes_slice(&self, len: usize) -> Result<&[u8]> {
        // TODO figure out why i can't call this method from deserialize_bytes_slice and just increment the indexif sucess
        match self.bytes.get(self.idx..self.idx + len) {
            Some(v) => Ok(v),
            // matched when N is greater then self.buffer.len()
            None => Err(SerDesError {
                message: format!(
                    "ByteDeserializer len: {len}, idx: {idx}, remaining: {rem}, requested: {req}",
                    len = self.len(),
                    rem = &self.remaining(),
                    req = len,
                    idx = self.idx,
                ),
            }),
        }
    }

    /// depletes `2` bytes for `u16`, etc. and returns after deserializing using `native` endianess
    /// FromNeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::ByteDeserializer;
    /// let mut des = ByteDeserializer::new(&[0x00, 0x01]);
    /// let v: u16 = des.deserialize_ne().unwrap();
    /// // ... etc
    /// ```    
    pub fn deserialize_ne<const N: usize, T: FromNeBytes<N, T>>(&mut self) -> Result<T> {
        let r = self.deserialize_bytes_array::<N>()?;
        Ok(T::from_bytes(r))
    }
    /// depletes `2` bytes for `u16`, etc. and returns after deserializing using `little` endianess
    /// FromLeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::ByteDeserializer;
    /// let mut des = ByteDeserializer::new(&[0x00, 0x01]);
    /// let v: u16 = des.deserialize_le().unwrap();
    /// // ... etc
    /// ```
    pub fn deserialize_le<const N: usize, T: FromLeBytes<N, T>>(&mut self) -> Result<T> {
        let r = self.deserialize_bytes_array::<N>()?;
        Ok(T::from_bytes(r))
    }
    /// depletes `2` bytes for `u16`, etc. and returns after deserializing using `big` endianess
    /// FromBeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::ByteDeserializer;
    /// let mut des = ByteDeserializer::new(&[0x00, 0x01]);
    /// let v: u16 = des.deserialize_be().unwrap();
    /// // ... etc
    /// ```
    pub fn deserialize_be<const N: usize, T: FromBeBytes<N, T>>(&mut self) -> Result<T> {
        let r = self.deserialize_bytes_array::<N>()?;
        Ok(T::from_bytes(r))
    }
    /// creates a new instance of `T` type `struct`, depleating exactly the right amount of bytes from [ByteDeserializer]
    /// `T` must implement [ByteDeserialize] trait
    pub fn deserialize<T>(&mut self) -> Result<T>
    where
        T: ByteDeserialize<T>,
    {
        T::byte_deserialize(self)
    }

    /// creates a new instance of T type struct, depleating `exactly` `len` bytes from [ByteDeserializer].
    /// Intended for types with variable length such as Strings, Vecs, etc.
    pub fn deserialize_take<T>(&mut self, len: usize) -> Result<T>
    where
        T: ByteDeserialize<T>,
    {
        T::byte_deserialize_take(self, len)
    }
}

/// This trait is to be implemented by any struct, example `MyFavStruct`, to be compatbile with [`ByteDeserializer::deserialize<MyFavStruct>()`]
pub trait ByteDeserialize<T> {
    /// If successfull returns a new instance of T type struct, depleating exactly the right amount of bytes from [ByteDeserializer]
    /// Number of bytes depleted is determined by the struct T itself and its member types.
    fn byte_deserialize(des: &mut ByteDeserializer) -> Result<T>;

    /// if sucessfull returns a new instance of T type struct, however ONLY depleating a maximum of `len` bytes from [ByteDeserializer]
    /// Intended for types with variable length such as Strings, Vecs, etc.
    /// No bytes will be depleted if attempt was not successful.
    fn byte_deserialize_take(des: &mut ByteDeserializer, len: usize) -> Result<T> {
        let bytes = des.peek_bytes_slice(len)?;
        let tmp_des = &mut ByteDeserializer::new(bytes);
        let result = Self::byte_deserialize(tmp_des);
        match result {
            Ok(v) => {
                des.advance_idx(bytes.len());
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }
}

/// Greedy deserialization of the remaining byte stream into a `Vec<u8>`
impl ByteDeserialize<Vec<u8>> for Vec<u8> {
    fn byte_deserialize(des: &mut ByteDeserializer) -> Result<Vec<u8>> {
        Ok(des.deserialize_bytes_slice_remaining().into())
    }
}

/// This is a short cut method that creates a new instance of [ByteDeserializer] and then uses that to convert them into a T type struct.
pub fn from_bytes<T>(bytes: &[u8]) -> Result<T>
where
    T: ByteDeserialize<T>,
{
    let de = &mut ByteDeserializer::new(bytes);
    T::byte_deserialize(de)
}

/// This is a short cut method that uses [`ByteSerializerStack<CAP>::as_slice()`] method to issue a [from_bytes] call.
pub fn from_serializer_stack<const CAP: usize, T>(ser: &ByteSerializerStack<CAP>) -> Result<T>
where
    T: ByteDeserialize<T>,
{
    from_bytes(ser.as_slice())
}
/// This is a short cut method that uses [`ByteSerializerHeap::as_slice()`] method to issue a [from_bytes] call.
pub fn from_serializer_heap<T>(ser: &ByteSerializerHeap) -> Result<T>
where
    T: ByteDeserialize<T>,
{
    from_bytes(ser.as_slice())
}

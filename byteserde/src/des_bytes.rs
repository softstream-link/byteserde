use std::fmt::{Debug, LowerHex};

use bytes::Bytes;

use crate::{
    error::SerDesError,
    utils::{
        hex::{to_hex_line, to_hex_pretty},
        numerics::{be_bytes::FromBeBytes, le_bytes::FromLeBytes, ne_bytes::FromNeBytes},
    },
};

/// Utility struct with a number of methods to enable deserialization of bytes into various types
/// ```
/// use ::byteserde::prelude::ByteDeserializerBytes;
/// let bytes = &[0x01, 0x00, 0x02, 0x00, 0x00, 0x03];
/// let mut des = ByteDeserializerBytes::new(bytes.to_vec().into());
/// assert_eq!(des.remaining(), 6);
/// assert_eq!(des.idx(), 0);
/// assert_eq!(des.len(), 6);
///
/// let first: u8 = des.deserialize_bytes_slice(1).unwrap()[0];
/// assert_eq!(first , 1);
///
/// let second: &[u8; 2] = des.deserialize_bytes_array_ref().unwrap();
/// assert_eq!(second, &[0x00, 0x02]);
///
/// let remaining: &[u8] = des.deserialize_bytes_slice_remaining();
/// assert_eq!(remaining, &[0x00, 0x00, 0x03]);
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct ByteDeserializerBytes {
    bytes: Bytes,
    idx: usize,
}

/// Provides a convenient way to view buffer content as both HEX and ASCII bytes where printable.
/// supports both forms of alternate
/// ```
/// use byteserde::des_bytes::ByteDeserializerBytes;
/// use bytes::Bytes;
///
/// let mut des = ByteDeserializerBytes::new(b"1234567890".as_ref().to_vec().into());
/// println ! ("{:#x}", des); // up to 16 bytes per line
/// println ! ("{:x}", des);  // single line
/// ```
impl LowerHex for ByteDeserializerBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = match f.alternate() {
            true => format!("\n{hex}", hex = to_hex_pretty(self.bytes.as_ref())),
            false => to_hex_line(self.bytes.as_ref()),
        };
        let len = self.bytes.len();
        let idx = self.idx;
        let rem = self.remaining();
        write!(f, "ByteDeserializerBytes {{ len: {len}, idx: {idx}, remaining: {rem}, bytes: {bytes} }}",)
    }
}

/// This is a unit testing only impl
impl From<Vec<u8>> for ByteDeserializerBytes {
    /// This is a unit testing only impl
    fn from(value: Vec<u8>) -> Self {
        ByteDeserializerBytes::new(Bytes::from(value))
    }
}

impl ByteDeserializerBytes {
    pub fn new(bytes: Bytes) -> ByteDeserializerBytes {
        ByteDeserializerBytes { bytes, idx: 0 }
    }

    /// Tracks the bytes read and always set to the next unread byte in the buffer. This is an inverse of [Self::remaining()]
    pub fn idx(&self) -> usize {
        self.idx
    }
    /// Number of bytes remaining to be deserialized, this is an inverse of [Self::idx()]
    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.idx
    }

    // // Number of bytes in the buffer does not change as deserialization progresses unlike [Self::remaining()] and [Self::idx()]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    #[cold]
    fn error(&self, n: usize) -> SerDesError {
        // moving error into a fn improves performance by 10%
        // from_bytes - reuse ByteDeserializerBytes
        // time:   [39.251 ns 39.333 ns 39.465 ns]
        // change: [-12.507% -11.603% -10.612%] (p = 0.00 < 0.05)
        // Performance has improved.
        SerDesError {
            message: format!("Failed to get a slice size: {n} bytes from {self:x}"),
        }
    }
    /// consumes all of the remaining bytes in the buffer and returns them as slice
    pub fn deserialize_bytes_slice_remaining(&mut self) -> &[u8] {
        // self.idx = self.bytes.len(); // consume all
        let res = &self.bytes[self.idx..];
        self.idx = self.len();
        res
    }
    /// consumes `len` bytes from the buffer and returns them as slice if successful.
    /// Fails if `len` is greater then [Self::remaining()]
    pub fn deserialize_bytes_slice(&mut self, len: usize) -> crate::error::Result<&[u8]> {
        match self.bytes.get(self.idx..self.idx + len) {
            Some(v) => {
                self.idx += len;
                Ok(v)
            }
            None => Err(self.error(len)),
        }
    }

    #[inline(always)]
    pub fn deserialize_u8(&mut self) -> crate::error::Result<u8> {
        let res = self.bytes.get(self.idx..);
        match res {
            Some(v) => {
                self.idx += 1;
                Ok(v[0])
            }
            None => Err(self.error(1)),
        }
    }
    #[inline(always)]
    pub fn deserialize_i8(&mut self) -> crate::error::Result<i8> {
        let res = self.bytes.get(self.idx..);
        match res {
            Some(v) => {
                self.idx += 1;
                Ok(v[0] as i8)
            }
            None => Err(self.error(1)),
        }
    }
    // /// moves the index forward by `len` bytes, intended to be used in combination with [Self::peek_bytes_slice()]
    fn advance_idx(&mut self, len: usize) {
        self.idx += len;
    }
    // /// produces with out consuming `len` bytes from the buffer and returns them as slice if successful.
    pub fn peek_bytes_slice(&self, len: usize) -> crate::error::Result<&[u8]> {
        // TODO figure out why i can't call this method from deserialize_bytes_slice and just increment the index if success
        match self.bytes.get(self.idx..self.idx + len) {
            Some(v) => Ok(v),
            None => Err(SerDesError {
                message: format!(
                    "ByteDeserializerBytes len: {len}, idx: {idx}, remaining: {rem}, requested: {req}, bytes:\n{self:#x}",
                    len = self.len(),
                    rem = &self.remaining(),
                    req = len,
                    idx = self.idx,
                ),
            }),
        }
    }
    pub fn peek_bytes(&self, at: usize) -> crate::error::Result<Bytes> {
        if self.remaining() > at {
            Err(self.error(self.idx + at))
        } else {
            Ok(self.bytes.clone().split_to(self.idx + at).split_off(self.idx))
        }
    }

    #[inline]
    pub fn deserialize_bytes_array_ref<const N: usize>(&mut self) -> crate::error::Result<&[u8; N]> {
        match self.bytes.get(self.idx..self.idx + N) {
            Some(v) => {
                self.idx += N;
                Ok(v.try_into().expect("Failed to convert &[u8] into &[u8; N]"))
            }
            None => Err(self.error(N)),
        }
    }
    /// depletes `2` bytes for `u16`, etc. and returns after deserializing using `native` endianess
    /// FromNeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::des_bytes::ByteDeserializerBytes;
    /// let mut des = ByteDeserializerBytes::new([0x00, 0x01].to_vec().into());
    /// let v: u16 = des.deserialize_ne().unwrap();
    /// ```
    #[inline]
    pub fn deserialize_ne<const N: usize, T: FromNeBytes<N, T>>(&mut self) -> crate::error::Result<T> {
        let r = self.deserialize_bytes_array_ref::<N>()?;
        Ok(T::from_bytes_ref(r))
    }
    /// depletes `2` bytes for `u16`, etc. and returns after deserializing using `little` endianess
    /// FromLeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::ByteDeserializerBytes;
    /// let mut des = ByteDeserializerBytes::new([0x01, 0x00].to_vec().into());
    /// let v: u16 = des.deserialize_le().unwrap();
    /// assert_eq!(v, 1);
    /// ```
    // #[inline]
    pub fn deserialize_le<const N: usize, T: FromLeBytes<N, T>>(&mut self) -> crate::error::Result<T> {
        let r = self.deserialize_bytes_array_ref::<N>()?;
        Ok(T::from_bytes_ref(r))
    }
    /// depletes `2` bytes for `u16`, etc. and returns after deserializing using `big` endianess
    /// FromBeBytes trait is already implemented for all rust's numeric primitives in this crate
    /// ```
    /// use ::byteserde::prelude::ByteDeserializerBytes;
    /// let mut des = ByteDeserializerBytes::new([0x00, 0x01].to_vec().into());
    /// let v: u16 = des.deserialize_be().unwrap();
    /// assert_eq!(v, 1);
    /// ```
    #[inline]
    pub fn deserialize_be<const N: usize, T: FromBeBytes<N, T>>(&mut self) -> crate::error::Result<T> {
        let r = self.deserialize_bytes_array_ref::<N>()?;
        Ok(T::from_bytes_ref(r))
    }
    /// creates a new instance of `T` type `struct`, depleting exactly the right amount of bytes from [ByteDeserializerBytes]
    /// `T` must implement [ByteDeserializeBytes] trait
    pub fn deserialize<T>(&mut self) -> crate::error::Result<T>
    where T: ByteDeserializeBytes<T> {
        T::byte_deserialize(self)
    }

    /// creates a new instance of T type struct, depleting `exactly` `len` bytes from [ByteDeserializerBytes].
    /// Intended for types with variable length such as Strings, Vec, etc.
    pub fn deserialize_take<T>(&mut self, len: usize) -> crate::error::Result<T>
    where T: ByteDeserializeBytes<T> {
        T::byte_deserialize_take(self, len)
    }
}

/// This trait is to be implemented by any struct, example `MyFavStruct`, to be compatible with [`ByteDeserializerBytes::deserialize<MyFavStruct>()`]
pub trait ByteDeserializeBytes<T> {
    /// If successful returns a new instance of T type struct, depleting exactly the right amount of bytes from [ByteDeserializerBytes]
    /// Number of bytes depleted is determined by the struct T itself and its member types.
    fn byte_deserialize(des: &mut ByteDeserializerBytes) -> crate::error::Result<T>;

    /// if successful returns a new instance of T type struct, however ONLY depleting a maximum of `len` bytes from [ByteDeserializerBytes]
    /// Intended for types with variable length such as Strings, Vec, etc.
    /// No bytes will be depleted if attempt was not successful.
    fn byte_deserialize_take(des: &mut ByteDeserializerBytes, len: usize) -> crate::error::Result<T> {
        let bytes = des.peek_bytes(len)?;
        let tmp_des = &mut ByteDeserializerBytes::new(bytes);
        let result = Self::byte_deserialize(tmp_des);
        match result {
            Ok(v) => {
                des.advance_idx(len);
                Ok(v)
            }
            Err(e) => Err(e),
        }
        // Err(SerDesError{message: "Not implemented yet".to_string()})
    }
}

/// Greedy deserialization of the remaining byte stream into a `Vec<u8>`
impl ByteDeserializeBytes<Bytes> for Bytes {
    fn byte_deserialize(des: &mut ByteDeserializerBytes) -> crate::error::Result<Bytes> {
        // println!("des.remaining() = {}", des.remaining());
        let bytes = des.peek_bytes(des.remaining())?;
        des.advance_idx(des.remaining());
        Ok(bytes)
    }
}
/// This is a short cut method that creates a new instance of [ByteDeserializerBytes] and then uses that to convert them into a T type struct.
pub fn from_bytes<T>(bytes: Bytes) -> crate::error::Result<T>
where T: ByteDeserializeBytes<T> {
    let de = &mut ByteDeserializerBytes::new(bytes);
    T::byte_deserialize(de)
}

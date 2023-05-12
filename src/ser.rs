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

/// A byte bufferallocated on stack with a fixed capacity, can be reused and recyled by calling [Self::reset()].
/// Returns SerDesError if required capacity is exceeded during serialization.
///
/// Example: Creates a buffer with 128 bytes capacity and serializes data into it.
/// ```
/// use byteserde::prelude::ByteSerializerStack;
/// let mut ser = ByteSerializerStack::<128>::default();
/// assert_eq!(ser.is_empty(), true);
///
/// ser.serialize_bytes(&[0x01]);
///
/// assert_eq!(ser.capacify(), 128);
/// assert_eq!(ser.avail(), 128 - 1);
/// assert_eq!(ser.len(), 1);
///
/// ser.reset();
/// assert_eq!(ser.is_empty(), true);
/// ```
#[derive(Debug, Clone)]
pub struct ByteSerializerStack<const CAP: usize> {
    bytes: [u8; CAP],
    len: usize,
}
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
    pub fn reset(&mut self) {
        self.len = 0;
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn capacify(&self) -> usize {
        CAP
    }
    pub fn avail(&self) -> usize {
        CAP - self.len
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[0..self.len]
    }

    pub fn serialize_bytes_array<const N: usize>(&mut self, bytes: &[u8; N]) -> Result<&mut Self> {
        self.serialize_bytes(bytes)?;
        Result::Ok(self)
    }
    pub fn serialize_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self> {
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
    pub fn serialize_ne<const N: usize, T: ToNeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes(&v.to_bytes())
    }
    pub fn serialize_le<const N: usize, T: ToLeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes(&v.to_bytes())
    }
    pub fn serialize_be<const N: usize, T: ToBeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes(&v.to_bytes())
    }
    pub fn serialize<T: ByteSerializeStack>(&mut self, v: &T) -> Result<&mut Self> {
        v.byte_serialize_stack(self)?;
        Ok(self)
    }
}

/// Trait type accepted by [ByteSerializerStack] for serialization.
///
/// Example: Define structure and manually implement ByteSerializeStack trait then use it to serialize.
/// ```
/// use byteserde::prelude::*;
///
/// struct MyStruct { a: u8, }
/// impl ByteSerializeStack for MyStruct {
///     fn byte_serialize_stack<const CAP: usize>(&self, ser: &mut ByteSerializerStack<CAP>) -> Result<()> {
///         ser.serialize_be(self.a)?;
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
///
/// ```
pub trait ByteSerializeStack {
    fn byte_serialize_stack<const CAP: usize>(
        &self,
        ser: &mut ByteSerializerStack<CAP>,
    ) -> Result<()>;
}

/// Analogous to [to_bytes_stack()], but returns an instance of [`ByteSerializerStack<CAP>`].
/// see [ByteSerializeStack] for example.
pub fn to_serializer_stack<const CAP: usize, T>(v: &T) -> Result<ByteSerializerStack<CAP>>
where
    T: ByteSerializeStack,
{
    let mut ser = ByteSerializerStack::<CAP>::default();
    v.byte_serialize_stack(&mut ser)?;
    Result::Ok(ser)
}
/// Analogous to [to_serializer_stack()], but returns just the array of bytes `[u8; CAP]`.
/// Note that this is not a `&[u8]` slice, but an array of bytes with length CAP even if
/// the actual length of the serialized data is less.
/// see [ByteSerializeStack] for example.
pub fn to_bytes_stack<const CAP: usize, T>(v: &T) -> Result<[u8; CAP]>
where
    T: ByteSerializeStack,
{
    let ser = to_serializer_stack(v)?;
    Ok(ser.bytes)
}

/////////////////////

pub trait ByteSerializeHeap {
    fn byte_serialize_heap(&self, ser: &mut ByteSerializerHeap) -> Result<()>;
}
#[derive(Debug, Default, Clone)]
pub struct ByteSerializerHeap {
    bytes: Vec<u8>,
}

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
    pub fn reset(&mut self) {
        self.bytes.clear();
    }
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.bytes.len() == 0
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[0..]
    }
    pub fn serialize_bytes_array<const N: usize>(&mut self, bytes: &[u8; N]) -> Result<&mut Self> {
        self.serialize_bytes(bytes)?;
        Result::Ok(self)
    }
    pub fn serialize_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self> {
        self.bytes.extend_from_slice(bytes);
        Ok(self)
    }
    pub fn serialize_ne<const N: usize, T: ToNeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes(&v.to_bytes())
    }
    pub fn serialize_le<const N: usize, T: ToLeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes(&v.to_bytes())
    }
    pub fn serialize_be<const N: usize, T: ToBeBytes<N>>(&mut self, v: T) -> Result<&mut Self> {
        self.serialize_bytes(&v.to_bytes())
    }

    pub fn serialize<T: ByteSerializeHeap>(&mut self, v: &T) -> Result<&mut Self> {
        v.byte_serialize_heap(self)?;
        Ok(self)
    }
}

pub fn to_serializer_heap<T>(v: &T) -> Result<ByteSerializerHeap>
where
    T: ByteSerializeHeap,
{
    let mut ser = ByteSerializerHeap::default();
    v.byte_serialize_heap(&mut ser)?;
    Result::Ok(ser)
}

pub fn to_bytes_heap<T>(v: &T) -> Result<Vec<u8>>
where
    T: ByteSerializeHeap,
{
    let ser = to_serializer_heap(v)?;
    Ok(ser.bytes)
}

use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};
use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Serialize, Deserialize)]
#[byteserde(endian = "le")]
pub struct Integers {
    type_i8: i8,
    #[byteserde(endian = "be")]
    type_u8: u8,
    type_i16: i16,
    #[byteserde(endian = "be")]
    type_u16: u16,
    type_i32: i32,
    #[byteserde(endian = "be")]
    type_u32: u32,
    type_i64: i64,
    #[byteserde(endian = "be")]
    type_u64: u64,
    type_i128: i128,
    #[byteserde(endian = "be")]
    type_u128: u128,
}
#[rustfmt::skip]
#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Serialize, Deserialize)]
#[byteserde(endian = "le")]
pub struct Floats {
    pub type_f32: f32,
    #[byteserde(endian = "be")]
    pub type_f64: f64,
}
#[rustfmt::skip]
#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Serialize, Deserialize)]
pub struct Numbers {
    pub type_header: Integers,
    pub type_footer: Floats,
}

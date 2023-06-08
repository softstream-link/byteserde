use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};
use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Serialize, Deserialize)]
// #[byteserde(endian = "le")]
pub struct Integers {
    type_i8: i8,
    type_u8: u8,
    type_i16: i16,
    type_u16: u16,
    type_i32: i32,
    type_u32: u32,
    type_i64: i64,
    type_u64: u64,
    type_i128: i128,
    type_u128: u128,
}
impl Default for Integers{
    fn default() -> Self {
        Self {
            type_i8: -8,
            type_u8: 8,
            type_i16: -16,
            type_u16: 16,
            type_i32: -32,
            type_u32: 32,
            type_i64: -64,
            type_u64: 64,
            type_i128: -128,
            type_u128: 128,
        }
    }
}
#[rustfmt::skip]
#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Serialize, Deserialize)]
// #[byteserde(endian = "le")]
pub struct Floats {
    pub type_f32: f32,
    pub type_f64: f64,
}
impl Default for Floats{
    fn default() -> Self {
        Self {
            type_f32: 1.32,
            type_f64: 2.64,
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Serialize, Deserialize)]
pub struct Numbers {
    pub type_header: Integers,
    pub type_footer: Floats,
}
impl Default for Numbers{
    fn default() -> Self {
        Self {
            type_header: Integers::default(),
            type_footer: Floats::default(),
        }
    }
}

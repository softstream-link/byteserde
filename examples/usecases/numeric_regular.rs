#![allow(unused_imports)] // supresses warnings in cargo run --example mode
use crate::unittest::setup;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};
use log::info;

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
struct Bytes {
    #[byteserde(replace(i8::MIN))]
    field_i8: i8,
    field_u8: u8,
}

#[test]
fn test_bytes() {
    setup::log::configure();
    let inp_bytes = Bytes {
        field_i8: -1,
        field_u8: 1,
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_bytes).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    assert_eq!(i8::MIN as u8, ser_stack.bytes()[0]);
    assert_eq!(0x01, ser_stack.bytes()[1]);

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_bytes).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_bytes: Bytes = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_bytes: {inp_bytes:?}");
    info!("out_bytes: {out_bytes:?}");
    assert_eq!(
        out_bytes,
        Bytes {
            field_i8: i8::MIN,
            field_u8: inp_bytes.field_u8,
        }
    );
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
#[byteserde(endian = "le")]
struct Numerics {
    #[byteserde(endian = "ne")] // ne test local attribute
    field_ne_local_macro: u16,

    #[byteserde(endian = "le")]
    field_le_local_macro: u16, // le test local attribute

    #[byteserde(endian = "be")]
    field_be_local_macro: u16, // be test local attribute
    field_be_global_macro: u16, // le test global attribute

    #[byteserde(replace(i16::MIN))]
    field_i16: i16,
    field_u16: u16,
    field_i32: i32,
    field_u32: u32,
    field_i64: i64,
    field_u64: u64,
    field_i128: i128,
    field_u128: u128,
    field_f32: f32,
    field_f64: f64,
}
#[test]
fn test_numerics() {
    setup::log::configure();

    let inp_num = Numerics {
        field_ne_local_macro: 0x00FF_u16,
        field_le_local_macro: 0x00FF_u16,
        field_be_local_macro: 0x00FF_u16,
        field_be_global_macro: 0x00FF_u16,
        field_i16: -1,
        field_i32: -1,
        field_i64: -1,
        field_i128: -1,
        ..Default::default()
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    let field_ne_local_macro = &ser_stack.bytes()[0..2];
    assert_eq!(field_ne_local_macro, 0x00FF_u16.to_ne_bytes());

    let field_le_local_macro = &ser_stack.bytes()[2..4];
    assert_eq!(field_le_local_macro, 0x00FF_u16.to_le_bytes());

    let field_be_local_macro = &ser_stack.bytes()[4..6];
    assert_eq!(field_be_local_macro, 0x00FF_u16.to_be_bytes());

    let field_be_global_macro = &ser_stack.bytes()[6..8];
    assert_eq!(field_be_global_macro, 0x00FF_u16.to_le_bytes());

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: Numerics = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_num: {inp_num:?}");
    info!("out_num: {out_num:?}");
    assert_eq!(inp_num, out_num);
}

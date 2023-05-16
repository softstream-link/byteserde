#![allow(unused_imports)] // supresses warnings in cargo run --example mode
use crate::unittest::setup;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};
use log::info;

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
#[byteserde(endian = "le")]
pub struct Numbers {
    #[byteserde(endian = "ne")] // ne test local attribute
    field_ne_local_macro: u16,

    #[byteserde(endian = "le")]
    field_le_local_macro: u16, // le test local attribute

    #[byteserde(endian = "be")]
    field_be_local_macro: u16, // be test local attribute
    field_be_global_macro: u16, // le test global attribute

    #[byteserde(endian = "be")]
    field_arr_u16_local_macro: [u16; 3], // be test local attribute
    field_arr_u16_global_macro: [u16; 3], // le test global attribute

    field_i8: i8,
    field_u8: u8,
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

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
pub struct Strings {
    field_string: String,
    field_char: char,
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
pub struct Nested {
    field_numbers: Numbers,
    field_strings: Strings,
}

#[test]
fn test_numbers() {
    setup::log::configure();

    let inp_num = Numbers {
        field_ne_local_macro: 0x00FF_u16,
        field_le_local_macro: 0x00FF_u16,
        field_be_local_macro: 0x00FF_u16,
        field_be_global_macro: 0x00FF_u16,
        field_arr_u16_local_macro: [0x0001_u16, 0x0002_u16, 0x0003_u16],
        field_arr_u16_global_macro: [0x0001_u16, 0x0002_u16, 0x0003_u16],
        ..Default::default()
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    let field_ne_local_macro = [ser_stack.bytes()[0], ser_stack.bytes()[1]];
    assert_eq!(field_ne_local_macro, 0x00FF_u16.to_ne_bytes());

    let field_le_local_macro = [ser_stack.bytes()[2], ser_stack.bytes()[3]];
    assert_eq!(field_le_local_macro, 0x00FF_u16.to_le_bytes());

    let field_be_local_macro = [ser_stack.bytes()[4], ser_stack.bytes()[5]];
    assert_eq!(field_be_local_macro, 0x00FF_u16.to_be_bytes());

    let field_be_global_macro = [ser_stack.bytes()[6], ser_stack.bytes()[7]];
    assert_eq!(field_be_global_macro, 0x00FF_u16.to_le_bytes());

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: Numbers = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(inp_num, out_num);
}

#[test]
fn test_strings() {
    setup::log::configure();

    let inp_str = Strings {
        field_string: "whatever".to_string(),
        field_char: '♥', // 3 bytes long
    };
    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
    info!("ser_stack: {ser_stack:#x}");
    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_str).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());
    // deserialize
    let out_str: Strings = from_serializer_heap(&ser_heap).unwrap();
    info!("inp: {inp_str:?}");
    info!("out: {out_str:?}");
    assert_eq!(inp_str, out_str);
}

#[test]
fn test_nested() {
    setup::log::configure();
    let inp_num = Numbers::default();
    let inp_str = Strings {
        field_string: "whatever".to_string(),
        field_char: '♥', // 3 bytes long
    };

    let inp_struct = Nested {
        field_numbers: inp_num,
        field_strings: inp_str,
    };
    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_struct).unwrap();
    info!("ser_stack: {ser_stack:#x}");
    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_struct).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    let out_struct: Nested = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_struct:?}");
    info!("out: {out_struct:?}");
    assert_eq!(inp_struct, out_struct);
}

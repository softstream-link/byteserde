mod unittest;
use std::mem::size_of;

use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};
use log::info;
use unittest::setup;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, 
        ByteSerializedSizeOf, ByteSerializedLenOf, Default, Debug, PartialEq)]
struct Bytes(#[byteserde(replace(i8::MIN))] i8, u8);

#[test]
fn test_bytes() {
    bytes()
}

fn bytes() {
    setup::log::configure();
    let inp_bytes = Bytes(-1, 1);

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_bytes).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    assert_eq!(i8::MIN as u8, ser_stack.as_slice()[0]);
    assert_eq!(0x01, ser_stack.as_slice()[1]);

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_bytes).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_bytes: Bytes = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_bytes: {inp_bytes:?}");
    info!("out_bytes: {out_bytes:?}");
    assert_eq!(out_bytes, Bytes(i8::MIN, inp_bytes.1,));
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Default, Debug, PartialEq)]
#[byteserde(endian = "le")]
struct Numerics(
    #[byteserde(endian = "ne")] // ne test local attribute
    u16,
    #[byteserde(endian = "le")] u16, // le test local attribute
    #[byteserde(endian = "be")] u16, // be test local attribute
    u16,                             // le test global attribute
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    u8,  // this shall cause alignment padding used in struct size_of test
    i128,
    u128,
    f32,
    f64,
);
#[test]
fn test_numerics() {
    numerics()
}
fn numerics() {
    setup::log::configure();

    let inp_num = Numerics(0x00FF_u16, 0x00FF_u16, 0x00FF_u16, 0x00FF_u16, -16, 16, -32, 32, -64, 64, 8_u8, -128, 128, -1.32, 1.64);

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    let field_ne_local_macro = &ser_stack.as_slice()[0..2];
    assert_eq!(field_ne_local_macro, 0x00FF_u16.to_ne_bytes());

    let field_le_local_macro = &ser_stack.as_slice()[2..4];
    assert_eq!(field_le_local_macro, 0x00FF_u16.to_le_bytes());

    let field_be_local_macro = &ser_stack.as_slice()[4..6];
    assert_eq!(field_be_local_macro, 0x00FF_u16.to_be_bytes());

    let field_be_global_macro = &ser_stack.as_slice()[6..8];
    assert_eq!(field_be_global_macro, 0x00FF_u16.to_le_bytes());

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_num: Numerics = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_num: {inp_num:?}");
    info!("out_num: {out_num:?}");
    assert_eq!(inp_num, out_num);
}

#[test]
fn test_size_and_len() {
    size_len();
}

fn size_len() {
    setup::log::configure();
    let ln_of = Bytes::default().byte_len();
    let sz_of = Bytes::byte_size();
    let sz_of_aligned = size_of::<Bytes>();
    info!("ln_of: {ln_of}");
    info!("sz_of: {sz_of}");
    info!("sz_of_aligned: {sz_of_aligned}");

    assert_eq!(ln_of, sz_of);
    assert_eq!(sz_of, size_of::<Bytes>());

    let ln_of = Numerics::default().byte_len();
    let sz_of = Numerics::byte_size();
    let sz_of_aligned = size_of::<Numerics>();
    info!("ln_of: {ln_of}");
    info!("sz_of: {sz_of}");
    info!("sz_of_aligned: {sz_of_aligned}");

    assert_eq!(ln_of, sz_of);
    assert_ne!(sz_of, sz_of_aligned);
    assert_eq!(sz_of, 81);
    assert_eq!(sz_of_aligned, 96);
}

fn main() {
    bytes();
    numerics();
    size_len()
}

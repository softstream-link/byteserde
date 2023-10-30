mod unittest;
use std::mem::size_of;

use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};
use log::info;
use unittest::setup;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice,
        ByteSerializedSizeOf, ByteSerializedLenOf, Default, Debug, PartialEq)]
struct UnitNothing;

#[test]
fn test_bytes() {
    bytes()
}
fn bytes() {
    setup::log::configure();

    let inp_num = UnitNothing::default();

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_num: UnitNothing = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(out_num, inp_num);
}

#[test]
fn test_bytes_size_len() {
    bytes_size_len()
}
fn bytes_size_len() {
    setup::log::configure();
    let ln_of = UnitNothing::default().byte_len();
    let sz_of = UnitNothing::byte_size();
    let sz_of_aligned = size_of::<UnitNothing>();
    info!("ln_of: {ln_of}");
    info!("sz_of: {sz_of}");
    info!("sz_of_aligned: {sz_of_aligned}");

    assert_eq!(ln_of, sz_of);
    assert_eq!(sz_of, sz_of_aligned);
}

fn main() {
    bytes();
    bytes_size_len();
}

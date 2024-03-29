mod unittest;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack};
use log::info;
use unittest::setup;
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, Debug, PartialEq)]
struct VecByte(#[byteserde(deplete(3))] Vec<u8>, #[byteserde(deplete(2), replace( vec![10,11] ))] Vec<u8>, Vec<u8>);

#[test]
fn test_vec_u8() {
    vec_u8()
}
fn vec_u8() {
    setup::log::configure();
    let inp_num = VecByte(vec![1, 2, 3], vec![], vec![6, 7, 8]);

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_num: VecByte = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    // let ser_num = VecByteStructRegular(inp_num.0);
    assert_eq!(out_num, VecByte(inp_num.0, vec![10, 11], inp_num.2,));
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, Debug, PartialEq)]
#[byteserde(endian = "le")]
struct VecNumerics(
    #[byteserde(endian = "be", deplete(3))] Vec<u16>,
    #[byteserde(deplete(2), replace( vec![10_u16, 11] ))] Vec<u16>,
    Vec<u16>,
);

#[test]
fn test_vec_u16() {
    vec_u16()
}
fn vec_u16() {
    setup::log::configure();
    let inp_num = VecNumerics(vec![1, 2, 3], vec![], vec![4, 5, 6]);

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");
    // head first
    assert_eq!(ser_stack.as_slice()[0..2], 1_u16.to_be_bytes());
    // tail first
    assert_eq!(ser_stack.as_slice()[10..12], 4_u16.to_le_bytes());

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_num: VecNumerics = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(out_num, VecNumerics(inp_num.0, vec![10, 11], inp_num.2,));
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, Debug, PartialEq, Default)]
struct Other(u8);
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, Debug, PartialEq)]
struct VecOther(
    #[byteserde(deplete(3))] Vec<Other>,
    #[byteserde(deplete(2), replace( vec![Other(10),Other(11)] ))] Vec<Other>,
    Vec<Other>,
);
#[test]
fn test_vec_other() {
    vec_other()
}
fn vec_other() {
    setup::log::configure();
    let inp_num = VecOther(vec![Other(1), Other(2), Other(3)], vec![], vec![Other(4), Other(5), Other(6)]);

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_num: VecOther = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(out_num, VecOther(inp_num.0, vec![Other(10), Other(11)], inp_num.2,));
}

fn main() {
    vec_u8();
    vec_u16();
    vec_other();
}

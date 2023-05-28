mod unittest;
use std::mem::size_of;

use byteserde::prelude::*;
use byteserde_derive::{
    ByteDeserialize, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf,
    ByteSerializedSizeOf,
};
use log::info;
use unittest::setup;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, 
        ByteSerializedSizeOf, ByteSerializedLenOf, Default, Debug, PartialEq)]
struct ArrBytes(
    [u8; 2],
    [i8; 2],
    #[byteserde(replace([10, 11]))] [u8; 2],
    #[byteserde(replace([-10, -11]))] [i8; 2],
);

#[test]
fn test_bytes() {
    bytes()
}
fn bytes() {
    setup::log::configure();
    let inp_num = ArrBytes([1, 2], [-1, -2], [0; 2], [0; 2]);

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_num: ArrBytes = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(
        out_num,
        ArrBytes(inp_num.0, inp_num.1, [10, 11], [-10, -11],)
    );
}
#[test]
fn test_bytes_size_len() {
    bytes_size_len()
}
fn bytes_size_len() {
    setup::log::configure();
    let ln_of = ArrBytes::default().byte_len();
    let sz_of = ArrBytes::byte_size();
    let sz_of_aligned = size_of::<ArrBytes>();
    info!("ln_of: {ln_of}");
    info!("sz_of: {sz_of}");
    info!("sz_of_aligned: {sz_of_aligned}");

    assert_eq!(ln_of, sz_of);
    assert_eq!(sz_of, sz_of_aligned);
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, 
        ByteSerializedSizeOf, ByteSerializedLenOf, Default, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct ArrNumerics(
    #[byteserde(endian = "ne")] [u16; 2],
    #[byteserde(endian = "le")] [u16; 2],
    #[byteserde(endian = "be")] [u16; 2],
    [u16; 2], // global macro
    #[byteserde(replace([10, 11]))] [u16; 2],
    [u8; 1], //field_arr_break_alignment: 
);

#[test]
fn test_numerics() {
    numerics()
}
fn numerics() {
    setup::log::configure();

    let inp_num = ArrNumerics([1, 2], [3, 4], [5, 6], [7, 8], [0; 2], [0; 1]);

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    assert_eq!(ser_stack.as_slice()[0..=1], 1_u16.to_ne_bytes());
    assert_eq!(ser_stack.as_slice()[4..=5], 3_u16.to_le_bytes());
    assert_eq!(ser_stack.as_slice()[8..=9], 5_u16.to_be_bytes());
    assert_eq!(ser_stack.as_slice()[12..=13], 7_u16.to_be_bytes());

    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_num: ArrNumerics = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(
        out_num,
        ArrNumerics(
            inp_num.0,
            inp_num.1,
            inp_num.2,
            inp_num.3,
            [10, 11],
            inp_num.5,
        )
    );
}
#[test]
fn test_numerics_size_len() {
    numerics_size_len()
}
fn numerics_size_len() {
    setup::log::configure();
    let ln_of = ArrNumerics::default().byte_len();
    let sz_of = ArrNumerics::byte_size();
    let sz_of_aligned = size_of::<ArrNumerics>();
    info!("ln_of: {ln_of}");
    info!("sz_of: {sz_of}");
    info!("sz_of_aligned: {sz_of_aligned}");

    assert_eq!(ln_of, sz_of);
    assert_ne!(sz_of, sz_of_aligned);
    assert_eq!(ln_of, 21);
    assert_eq!(sz_of_aligned, 22);
}
#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize,
        ByteSerializedSizeOf, ByteSerializedLenOf, Default, Debug, PartialEq, Copy, Clone)]
struct Other(u8);

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, 
        ByteSerializedSizeOf, ByteSerializedLenOf, Default, Debug, PartialEq, Copy, Clone)]
struct OtherBreakAlignment(u16, u8);

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, 
        ByteSerializedSizeOf, ByteSerializedLenOf, Default, Debug, PartialEq)]
struct ArrOther(
    [Other; 2],
    #[byteserde(replace([Other(3), Other(4)]))] [Other; 2],
    [OtherBreakAlignment; 1],
);

#[test]
fn test_other() {
    other()
}
fn other() {
    setup::log::configure();

    let inp_other = ArrOther([Other::default(); 2], [Other(1), Other(2)], [OtherBreakAlignment(0, 0)]);

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_other).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_other).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_other: ArrOther = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_other: {inp_other:?}");
    info!("out_other: {out_other:?}");
    assert_eq!(out_other, ArrOther(inp_other.0, [Other(3), Other(4)], inp_other.2 ));
}
#[test]
fn test_other_size_len() {
    other_size_len()
}

fn other_size_len(){
    setup::log::configure();
    let ln_of = ArrOther::default().byte_len();
    let sz_of = ArrOther::byte_size();
    let sz_of_aligned = size_of::<ArrOther>();
    info!("ln_of: {ln_of}");
    info!("sz_of: {sz_of}");
    info!("sz_of_aligned: {sz_of_aligned}");

    assert_eq!(ln_of, sz_of);
    assert_ne!(sz_of, sz_of_aligned);
    assert_eq!(ln_of, 7);
    assert_eq!(sz_of_aligned, 8);
}

fn main() {
    bytes();
    bytes_size_len();
    numerics();
    numerics_size_len();
    other();
    other_size_len();
}

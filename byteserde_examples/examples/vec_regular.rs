mod unittest;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf};
use log::info;
use unittest::setup;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, 
        ByteSerializedLenOf, Debug, PartialEq)]
struct VecByte {
    #[byteserde(deplete(3))]
    field_vec_u8_head: Vec<u8>,  // will ser/der 3
    #[byteserde(deplete(2), replace( vec![10,11, 13] ))] // 13 gets dropped
    field_vec_u8_body: Vec<u8>, // will ser/der 2 
    field_vec_u8_tail: Vec<u8>,
}
impl Default for VecByte {
    fn default() -> Self {
        VecByte {
            field_vec_u8_head: vec![1, 2, 3, 13], // 13 gets dropped
            field_vec_u8_body: vec![],
            field_vec_u8_tail: vec![6, 7, 8],
        }
    }
}

//TODO CRITICAL Bytes example and core test

#[test]
#[should_panic(expected = "VecByte.field_vec_u8_head field #[byteserde(deplete( .. ))] set higher then length of Vec instance")]
fn vec_u8_deplete_invalid() {
    setup::log::configure();
    let mut inp_num = VecByte::default();
    inp_num.field_vec_u8_head = vec![1, 2];
    let _: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
}

#[test]
fn test_vec_u8() {
    vec_u8()
}
fn vec_u8() {
    setup::log::configure();
    let inp_num = VecByte::default();

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
    assert_eq!(
        out_num,
        VecByte {
            field_vec_u8_head: vec![1, 2, 3], // 13 got dropped
            field_vec_u8_body: vec![10, 11],  // 13 got dropped
            ..inp_num
        }
    );
}
#[test]
fn test_vec_len() {
    vec_len()
}
fn vec_len() {
    setup::log::configure();
    let inp_num = VecByte::default();
    let inp_num_len = inp_num.byte_len();
    info!("inp_num: {:?}", inp_num);
    info!("inp_num_len: {}", inp_num_len);
    // 3*u8=3 via deplete +
    // 2*u8=2 via deplete +
    // 3*u8=3 via len  = 8
    assert_eq!(inp_num_len, 8);
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, 
        ByteSerializedLenOf, Debug, PartialEq)]
#[byteserde(endian = "le")]
struct VecNumerics {
    #[byteserde(endian = "be", deplete(3))] 
    field_vec_u16_head: Vec<u16>, // will ser/des 3 u16 bytes
    #[byteserde(deplete(2), replace( vec![7_u32, 8, 13] ))] // 13 gets dropped
    field_vec_u32_body: Vec<u32>, // will ser/des 2 u32 bytes
    field_vec_u64_tail: Vec<u64>, // will greedily ser/des all avail u64 bytes
}
impl Default for VecNumerics {
    fn default() -> Self {
        VecNumerics {
            field_vec_u16_head: vec![1, 2, 3, 13], // 13 gets dropped
            field_vec_u32_body: vec![],
            field_vec_u64_tail: vec![4, 5, 6],
        }
    }
}
#[test]
fn test_vec_numeric() {
    vec_numeric()
}
fn vec_numeric() {
    setup::log::configure();
    let inp_num = VecNumerics::default();

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");
    // head be
    assert_eq!(ser_stack.as_slice()[0..=1], 1_u16.to_be_bytes());
    // body le
    assert_eq!(ser_stack.as_slice()[6..=9], 7_u32.to_le_bytes());

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_num: VecNumerics = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(
        out_num,
        VecNumerics {
            field_vec_u16_head: vec![1, 2, 3],  // 13 was dropped
            field_vec_u32_body: vec![7_u32, 8], // 13 was dropped
            ..inp_num
        }
    );
}
#[test]
fn test_vec_numeric_len() {
    vec_numeric_len()
}
fn vec_numeric_len() {
    setup::log::configure();
    let inp_num = VecNumerics::default();
    let inp_num_len = inp_num.byte_len();
    info!("inp_num: {:?}", inp_num);
    info!("inp_num_len: {}", inp_num_len);
    // 3*u16=6 via deplete +
    // 2*u32=8 via deplete +
    // 3*u64=24 via len = 38
    assert_eq!(inp_num_len, 38);
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf, Debug, PartialEq, Default)]
struct Other(u8);

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf, Debug, PartialEq)]
struct VecOther {
    #[byteserde(deplete(3))]  // will only ser/des 3 Other Instances
    field_vec_other_head: Vec<Other>,
    #[byteserde(deplete(2), replace( vec![Other(10),Other(11), Other(13)] ))] // Other(13) will be dropped
    field_vec_other_body: Vec<Other>,
    field_vec_other_tail: Vec<Other>, // will greedily ser/des all avail Other Instances
}
impl Default for VecOther {
    fn default() -> Self {
        VecOther {
            field_vec_other_head: vec![Other(1), Other(2), Other(3), Other(13)], // Other(13) will be dropped
            field_vec_other_body: vec![],
            field_vec_other_tail: vec![Other(4), Other(5), Other(6)],
        }
    }
}
#[test]
fn test_vec_other() {
    vec_other()
}
fn vec_other() {
    setup::log::configure();
    let inp_num = VecOther::default();

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
    assert_eq!(
        out_num,
        VecOther {
            field_vec_other_head: vec![Other(1), Other(2), Other(3)], // Other(13) was dropped
            field_vec_other_body: vec![Other(10), Other(11)],
            ..inp_num
        }
    );
}
#[test]
fn test_vec_other_len() {
    vec_other_len()
}

fn vec_other_len() {
    setup::log::configure();
    let inp_num = VecOther::default();
    let inp_num_len = inp_num.byte_len();
    info!("inp_num: {:?}", inp_num);
    info!("inp_num_len: {}", inp_num_len);
    // 3*Other(u8)=3 via deplete +
    // 2*Other(u8)=2 via deplete & replace +
    // 3*Other(u8)=3 via len = 8
    assert_eq!(inp_num_len, 8);
}

fn main() {
    vec_u8();
    vec_len();
    vec_numeric();
    vec_numeric_len();
    vec_other();
    vec_other_len();
}

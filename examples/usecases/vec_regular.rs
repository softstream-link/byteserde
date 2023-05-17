use byteserde::prelude::*;

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
struct VecByte {
    #[byteserde(length(3))]
    field_vec_u8_head: Vec<u8>,
    #[byteserde(length(2), replace( vec![10,11] ))]
    field_vec_u8_body: Vec<u8>,
    field_vec_u8_tail: Vec<u8>,
}

#[test]
fn vec_u8() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();
    let inp_num = VecByte {
        field_vec_u8_head: vec![1, 2, 3],
        field_vec_u8_body: vec![],
        field_vec_u8_tail: vec![6, 7, 8],
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: VecByte = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(
        out_num,
        VecByte {
            field_vec_u8_body: vec![10, 11],
            ..inp_num
        }
    );
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
#[byteserde(endian = "le")]
struct VecNumerics {
    #[byteserde(endian = "be", length(3))]
    field_vec_u16_head: Vec<u16>,
    #[byteserde(length(2), replace( vec![10_u16, 11] ))]
    field_vec_u16_body: Vec<u16>,
    field_vec_u16_tail: Vec<u16>,
}

#[test]
fn vec_u16() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();
    let inp_num = VecNumerics {
        field_vec_u16_head: vec![1, 2, 3],
        field_vec_u16_body: vec![],
        field_vec_u16_tail: vec![4, 5, 6],
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");
    // head first
    assert_eq!(ser_stack.bytes()[0..2], 1_u16.to_be_bytes());
    // tail first
    assert_eq!(ser_stack.bytes()[10..12], 4_u16.to_le_bytes());

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: VecNumerics = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(
        out_num,
        VecNumerics {
            field_vec_u16_body: vec![10, 11],
            ..inp_num
        }
    );
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq, Default)]
struct Other(u8);
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
struct VecOther {
    #[byteserde(length(3))]
    field_vec_other_head: Vec<Other>,
    #[byteserde(length(2), replace( vec![Other(10),Other(11)] ))]
    field_vec_other_body: Vec<Other>,
    field_vec_other_tail: Vec<Other>,
}
#[test]
fn vec_other() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();
    let inp_num = VecOther {
        field_vec_other_head: vec![Other(1), Other(2), Other(3)],
        field_vec_other_body: vec![],
        field_vec_other_tail: vec![Other(4), Other(5), Other(6)],
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: VecOther = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(
        out_num,
        VecOther {
            field_vec_other_body: vec![Other(10), Other(11)],
            ..inp_num
        }
    );
}

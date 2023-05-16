use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
struct Bytes(#[byteserde(replace(i16::MIN))] i8, u8);

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
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
    i128,
    u128,
    f32,
    f64,
);

#[test]
fn test_bytes() {
    use crate::unittest::setup;
    use byteserde::prelude::*;
    use log::info;
    setup::log::configure();
    let inp_bytes = Bytes(-1, 1);

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_bytes).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    assert_eq!(0xff, ser_stack.bytes()[0]);
    assert_eq!(0x01, ser_stack.bytes()[1]);

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_bytes).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_bytes: Bytes = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_bytes: {inp_bytes:?}");
    info!("out_bytes: {out_bytes:?}");
    assert_eq!(out_bytes, Bytes(i8::MIN, inp_bytes.1,));
}

#[test]
fn test_numerics() {
    use crate::unittest::setup;
    use byteserde::prelude::*;
    use log::info;
    setup::log::configure();

    let inp_num = Numerics(
        0x00FF_u16, 0x00FF_u16, 0x00FF_u16, 0x00FF_u16, -16, 16, -32, 32, -64, 64, -128, 128,
        -1.32, 1.64,
    );

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

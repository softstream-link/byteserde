use byteserde::prelude::*;
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq, Clone)]
#[byteserde(endian = "le")]
struct NumbersStructTuple(
    #[byteserde(endian = "ne")] u16,
    #[byteserde(endian = "le")] u16,
    #[byteserde(endian = "be")] u16,
    u16, // global macro endian
    #[byteserde(endian = "be")] [u16; 3],
    [u16; 3], // global macro endian
    i8,
    u8,
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

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq, Clone)]
pub struct StringsStructTuple(String, char);

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
pub struct NestedStructTuple(NumbersStructTuple, StringsStructTuple);

#[test]
fn all() {
    // ********************** NUMERICS **********************
    let inp_num = NumbersStructTuple(
        0x00FF_u16,
        0x00FF_u16,
        0x00FF_u16,
        0x00FF_u16,
        [0x0001_u16, 0x0002_u16, 0x0003_u16],
        [0x0001_u16, 0x0002_u16, 0x0003_u16],
        -1,
        1,
        -2,
        2,
        -3,
        3,
        -4,
        4,
        -5,
        5,
        -6.2,
        6.2,
    );
    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    println!("ser_stack: {ser_stack:#x}");
    let field_0_ne_local_macro = [ser_stack.bytes()[0], ser_stack.bytes()[1]];
    assert_eq!(field_0_ne_local_macro, 0x00FF_u16.to_ne_bytes());

    let field_1_le_local_macro = [ser_stack.bytes()[2], ser_stack.bytes()[3]];
    assert_eq!(field_1_le_local_macro, 0x00FF_u16.to_le_bytes());

    let field_2_be_local_macro = [ser_stack.bytes()[4], ser_stack.bytes()[5]];
    assert_eq!(field_2_be_local_macro, 0x00FF_u16.to_be_bytes());

    let field_3_be_global_macro = [ser_stack.bytes()[6], ser_stack.bytes()[7]];
    assert_eq!(field_3_be_global_macro, 0x00FF_u16.to_le_bytes());

    //heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    println!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: NumbersStructTuple = from_serializer_stack(&ser_stack).unwrap();
    println!("inp_num: {inp_num:?}");
    println!("out_num: {out_num:?}");
    assert_eq!(inp_num, out_num);

    // ********************** STRINGS **********************
    let inp_str = StringsStructTuple(
        "whatever".to_string(),
        '♥', // 3 bytes long
    );
    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
    println!("ser_stack: {ser_stack:#x}");
    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_str).unwrap();
    println!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());
    // deserialize
    let out_str: StringsStructTuple = from_serializer_stack(&ser_stack).unwrap();
    println!("inp_str: {inp_str:?}");
    println!("out_str: {out_str:?}");
    assert_eq!(inp_str, out_str);

    // ********************** NESTED **********************
    let inp_struct = NestedStructTuple(inp_num.clone(), inp_str.clone());
    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_struct).unwrap();
    println!("ser_stack: {ser_stack:#x}");
    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_struct).unwrap();
    println!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());
    // deserialize
    let out_struct: NestedStructTuple = from_serializer_stack(&ser_stack).unwrap();
    println!("inp_struct: {inp_struct:?}");
    println!("out_struct: {out_struct:?}");
    assert_eq!(inp_struct, out_struct);
}

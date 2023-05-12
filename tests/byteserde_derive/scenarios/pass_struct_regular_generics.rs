use byteserde::prelude::*;
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq, Clone)]
#[byteserde(endian = "le")]
pub struct NumbersStructRegular<const L: usize, const M: usize> {
    #[byteserde(endian = "be")]
    filed_arr_u16_local_macro: [u16; L],
    filed_arr_u16_global_macro: [u16; M],
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq, Clone)]
pub struct StringsStructRegular<
    S: ByteSerializeStack + ByteSerializeHeap + ByteDeserialize<S>,
    C: ByteSerializeStack + ByteSerializeHeap + ByteDeserialize<C>,
> {
    field_string: S,
    field_char: C,
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
pub struct NestedStructRegular<
    const L: usize,
    const M: usize,
    S: ByteSerializeStack + ByteSerializeHeap + ByteDeserialize<S>,
    C: ByteSerializeStack + ByteSerializeHeap + ByteDeserialize<C>,
> {
    field_numbers: NumbersStructRegular<L, M>,
    field_strings: StringsStructRegular<S, C>,
}

fn main() {
    // **************** NUMERICS ****************
    let inp_num = NumbersStructRegular::<2, 3> {
        filed_arr_u16_local_macro: [0x0001_u16, 0x0002_u16],
        filed_arr_u16_global_macro: [0x0001_u16, 0x0002_u16, 0x0003_u16],
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    println!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    println!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: NumbersStructRegular<2, 3> = from_serializer_stack(&ser_stack).unwrap();
    println!("inp_num: {inp_num:?}");
    println!("out_num: {out_num:?}");
    assert_eq!(inp_num, out_num);

    // **************** STRINGS ****************
    // let inp_str = StringsStructRegular::<String, char> {
    let inp_str = StringsStructRegular::<String, char> {
        field_string: "Hello".to_string(),
        field_char: 'a',
    };
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
    println!("ser_stack: {ser_stack:#x}");

    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_str).unwrap();
    println!("ser_heap: {ser_heap:#x}");

    let out_str = from_serializer_stack(&ser_stack).unwrap();
    println!("inp_str: {inp_str:?}");
    println!("out_str: {out_str:?}");
    assert_eq!(inp_str, out_str);

    // **************** NESTED ****************
    let inp_nes = NestedStructRegular::<2, 3, String, char> {
        field_numbers: inp_num.clone(),
        field_strings: inp_str.clone(),
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_nes).unwrap();
    println!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_nes).unwrap();
    println!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_nes: NestedStructRegular<2, 3, String, char> =
        from_serializer_stack(&ser_stack).unwrap();
    println!("inp_nes: {inp_nes:?}");
    println!("out_nes: {out_nes:?}");
    assert_eq!(inp_nes, out_nes);
}

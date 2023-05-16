use byteserde::prelude::*;
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};
use log::info;

use crate::unittest::setup;

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq, Clone)]
#[byteserde(endian = "le")]
pub struct NumbersStructRegular<const L: usize, const M: usize>(
    #[byteserde(endian = "be")] [u16; L],
    [u16; M],
);

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq, Clone)]
pub struct StringsStructRegular<
    S: ByteSerializeStack + ByteSerializeHeap + ByteDeserialize<S>,
    C: ByteSerializeStack + ByteSerializeHeap + ByteDeserialize<C>,
>(S, C);

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
pub struct NestedStructRegular<
    const L: usize,
    const M: usize,
    S: ByteSerializeStack + ByteSerializeHeap + ByteDeserialize<S>,
    C: ByteSerializeStack + ByteSerializeHeap + ByteDeserialize<C>,
>(NumbersStructRegular<L, M>, StringsStructRegular<S, C>);

#[test]
fn all() {
    setup::log::configure();
    // **************** NUMERICS ****************
    let inp_num = NumbersStructRegular::<2, 3>(
        [0x0001_u16, 0x0002_u16],
        [0x0001_u16, 0x0002_u16, 0x0003_u16],
    );

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: NumbersStructRegular<2, 3> = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_num: {inp_num:?}");
    info!("out_num: {out_num:?}");
    assert_eq!(inp_num, out_num);

    // **************** STRINGS ****************
    let inp_str = StringsStructRegular::<String, char>("Hello".to_string(), 'a');
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_str).unwrap();
    info!("ser_heap: {ser_heap:#x}");

    let out_str = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_str: {inp_str:?}");
    info!("out_str: {out_str:?}");
    assert_eq!(inp_str, out_str);

    // **************** NESTED ****************
    let inp_nes = NestedStructRegular::<2, 3, String, char>(inp_num.clone(), inp_str.clone());

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_nes).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_nes).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_nes: NestedStructRegular<2, 3, String, char> =
        from_serializer_stack(&ser_stack).unwrap();
    info!("inp_nes: {inp_nes:?}");
    info!("out_nes: {out_nes:?}");
    assert_eq!(inp_nes, out_nes);
}

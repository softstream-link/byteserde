use byteserde::prelude::*;

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
struct ArrBytes {
    field_arr_u8: [u8; 2],
    field_arr_i8: [i8; 2],
    #[byteserde(replace([10, 11]))]
    field_arr_u8_repl: [u8; 2],
    #[byteserde(replace([-10, -11]))]
    field_arr_i8_repl: [i8; 2],
}

#[test]
fn test_bytes() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();

    let inp_num = ArrBytes {
        field_arr_u8: [1, 2],
        field_arr_i8: [-1, -2],
        ..Default::default()
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: ArrBytes = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(
        out_num,
        ArrBytes {
            field_arr_u8_repl: [10, 11],
            field_arr_i8_repl: [-10, -11],
            ..inp_num
        }
    );
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct ArrNumerics {
    #[byteserde(endian = "ne")]
    field_arr_ne_local_macro: [u16; 2],
    #[byteserde(endian = "le")]
    field_arr_le_local_macro: [u16; 2],
    #[byteserde(endian = "be")]
    field_arr_be_local_macro: [u16; 2],
    field_arr_be_global_macro: [u16; 2], // global macro
    #[byteserde(replace([10, 11]))]
    field_arr_relp: [u16; 2],
}

#[test]
fn test_numerics() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();

    let inp_num = ArrNumerics {
        field_arr_ne_local_macro: [1, 2],
        field_arr_le_local_macro: [3, 4],
        field_arr_be_local_macro: [5, 6],
        field_arr_be_global_macro: [7, 8],
        ..Default::default()
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    assert_eq!(ser_stack.bytes()[0..=1], 1_u16.to_ne_bytes());
    assert_eq!(ser_stack.bytes()[4..=5], 3_u16.to_le_bytes());
    assert_eq!(ser_stack.bytes()[8..=9], 5_u16.to_be_bytes());
    assert_eq!(ser_stack.bytes()[12..=13], 7_u16.to_be_bytes());

    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_num: ArrNumerics = from_serializer_stack(&ser_stack).unwrap();
    info!("inp: {inp_num:?}");
    info!("out: {out_num:?}");
    assert_eq!(
        out_num,
        ArrNumerics {
            field_arr_relp: [10, 11],
            ..inp_num
        }
    );
}

#[derive(
    ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq, Copy, Clone,
)]
struct Other(u8);

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
struct ArrOther {
    field_arr_other: [Other; 2],
    #[byteserde(replace([Other(3), Other(4)]))]
    filed_arr_other_repl: [Other; 2],
}

#[test]
fn test_other() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();

    let inp_other = ArrOther {
        field_arr_other: [Other(1), Other(2)],
        ..Default::default()
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_other).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_other).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    // deserialize
    let out_other: ArrOther = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_other: {inp_other:?}");
    info!("out_other: {out_other:?}");
    assert_eq!(
        out_other,
        ArrOther {
            filed_arr_other_repl: [Other(3), Other(4)],
            ..inp_other
        }
    );
}

use byteserde::prelude::*;

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
struct ArrBytes(
    [u8; 2],
    [i8; 2],
    #[byteserde(replace([10, 11]))] [u8; 2],
    #[byteserde(replace([-10, -11]))] [i8; 2],
);

#[test]
fn test_bytes() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();

    let inp_num = ArrBytes([1, 2], [-1, -2], [0; 2], [0; 2]);

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
        ArrBytes(inp_num.0, inp_num.1, [10, 11], [-10, -11],)
    );
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct ArrNumerics(
    #[byteserde(endian = "ne")] [u16; 2],
    #[byteserde(endian = "le")] [u16; 2],
    #[byteserde(endian = "be")] [u16; 2],
    [u16; 2], // global macro
    #[byteserde(replace([10, 11]))] [u16; 2],
);

#[test]
fn test_numerics() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();

    let inp_num = ArrNumerics([1, 2], [3, 4], [5, 6], [7, 8], [0; 2]);

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
        ArrNumerics(inp_num.0, inp_num.1, inp_num.2, inp_num.3, [10, 11],)
    );
}

#[derive(
    ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq, Copy, Clone,
)]
struct Other(u8);

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default, Debug, PartialEq)]
struct ArrOther(
    [Other; 2],
    #[byteserde(replace([Other(3), Other(4)]))] [Other; 2],
);

#[test]
fn test_other() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();

    let inp_other = ArrOther([Other::default(); 2], [Other(1), Other(2)]);

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
    assert_eq!(out_other, ArrOther(inp_other.0, [Other(3), Other(4)],));
}

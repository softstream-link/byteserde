use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
struct Strings(String, char);

#[test]
fn test_strings() {
    use crate::unittest::setup;
    use byteserde::prelude::*;
    use log::info;
    setup::log::configure();

    let inp_str = Strings(
        "whatever".to_string(),
        '♥', // 3 bytes long
    );

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
    info!("ser_stack: {ser_stack:#x}");
    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_str).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());
    // deserialize
    let out_str: Strings = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_str: {:?}", inp_str);
    info!("out_str: {:?}", out_str);
    assert_eq!(inp_str, out_str);
}

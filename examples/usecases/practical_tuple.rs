use byteserde::utils::strings::ascii::{ConstCharAscii, StringAscii};
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};

type Plus = ConstCharAscii<b'+'>;
#[derive(ByteDeserialize, ByteSerializeStack, ByteSerializeHeap, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct DebugMsg(
    #[byteserde(replace( (_1.len() + _2.len()) as u16 ))] u16,
    Plus,
    #[byteserde(length ( _0 as usize - _1.len() ))] StringAscii,
);

impl Default for DebugMsg {
    fn default() -> Self {
        Self(Default::default(), Default::default(), b"0123456789".into())
    }
}

#[test]
fn all() {
    use crate::unittest::setup;
    use byteserde::prelude::*;
    use log::info;
    setup::log::configure();
    let inp_debug = DebugMsg::default();
    info!("inp_debug: {:?}", inp_debug);

    let tail = &[0x01, 0x02, 0x3];
    // stack
    let mut ser_stack: ByteSerializerStack<135> = to_serializer_stack(&inp_debug).unwrap();
    ser_stack.serialize_bytes(tail).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let mut ser_heap = to_serializer_heap(&inp_debug).unwrap();
    ser_heap.serialize_bytes(tail).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    let des = &mut ByteDeserializer::new(ser_stack.bytes());

    let out_debug = DebugMsg::byte_deserialize(des).unwrap();
    info!("out_debug: {:?}", out_debug);
    info!("des: {:#x}", des);

    assert_eq!(inp_debug.0 + 11, out_debug.0);
    assert_eq!(inp_debug.1, out_debug.1);
    assert_eq!(inp_debug.2, out_debug.2);
    assert_eq!(des.remaining(), tail.len());
}

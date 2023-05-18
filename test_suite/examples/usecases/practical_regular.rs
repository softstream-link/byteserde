use byteserde::prelude::*;
use byteserde::utils::strings::ascii::{ConstCharAscii, StringAscii};

type Plus = ConstCharAscii<b'+'>;

#[derive(ByteDeserialize, ByteSerializeStack, ByteSerializeHeap, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct DebugMsg {
    #[byteserde(replace( (text.len() + packet_type.len()) as u16 ))]
    packet_length: u16,
    packet_type: Plus,
    #[byteserde(deplete( packet_length as usize - packet_type.len() ))]
    text: StringAscii,
}

impl Default for DebugMsg {
    fn default() -> Self {
        Self {
            packet_length: Default::default(),
            packet_type: Default::default(),
            text: b"0123456789".into(),
        }
    }
}

#[test]
fn all() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();
    let inp_debug = DebugMsg::default();
    info!("inp_debug: {:?}", inp_debug);

    let tail = &[0x01, 0x02, 0x3];
    // stack
    let mut ser_stack: ByteSerializerStack<135> = to_serializer_stack(&inp_debug).unwrap();
    ser_stack.serialize_bytes_slice(tail).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let mut ser_heap = to_serializer_heap(&inp_debug).unwrap();
    ser_heap.serialize_bytes_slice(tail).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    let des = &mut ByteDeserializer::new(ser_stack.bytes());

    let out_debug = DebugMsg::byte_deserialize(des).unwrap();
    info!("out_debug: {:?}", out_debug);
    info!("des: {:#x}", des);

    assert_eq!(inp_debug.packet_length + 11, out_debug.packet_length);
    assert_eq!(inp_debug.packet_type, out_debug.packet_type);
    assert_eq!(inp_debug.text, out_debug.text);
    assert_eq!(des.remaining(), tail.len());
}

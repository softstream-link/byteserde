use byteserde::{
    des::ByteDeserialize,
    prelude::ByteDeserializer,
    ser::{to_serializer_heap, to_serializer_stack, ByteSerializerStack},
    utils::strings::ascii::{ConstCharAscii, StringAscii},
};
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};

#[derive(ByteDeserialize, ByteSerializeStack, ByteSerializeHeap, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct DebugMsg {
    #[byteserde(replace( (text.len() + packet_type.len()) as u16 ))]
    packet_length: u16,
    packet_type: ConstCharAscii<b'+'>,
    #[byteserde(length ( packet_length as usize - 1 ))]
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

fn main() {
    let inp_debug = DebugMsg::default();
    println!("inp_debug: {:?}", inp_debug);

    let tail = &[0x01, 0x02, 0x3];
    // stack
    let mut ser_stack: ByteSerializerStack<135> = to_serializer_stack(&inp_debug).unwrap();
    ser_stack.serialize_bytes_array(tail).unwrap();
    println!("ser_stack: {ser_stack:#x}");

    // heap
    let mut ser_heap = to_serializer_heap(&inp_debug).unwrap();
    ser_heap.serialize_bytes_array(tail).unwrap();
    println!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    let des = &mut ByteDeserializer::new(ser_stack.bytes());

    let out_debug = DebugMsg::byte_deserialize(des).unwrap();
    println!("out_debug: {:?}", out_debug);
    println!("des: {:#x}", des);

    assert_eq!(inp_debug.packet_length + 11, out_debug.packet_length);
    assert_eq!(inp_debug.packet_type, out_debug.packet_type);
    assert_eq!(inp_debug.text, out_debug.text);
    assert_eq!(des.remaining(), tail.len());
}

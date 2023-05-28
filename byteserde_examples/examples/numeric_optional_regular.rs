mod unittest;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack, ByteSerializedSizeOf};
use byteserde_types::prelude::*;
use log::info;
use unittest::setup;

// use std::mem::size_of;

// #[derive(ByteSerializeStack, Debug, PartialEq)]
#[derive(ByteSerializedSizeOf, Debug, PartialEq)]
struct OptionalNumeric {
    // #[byteserde(replace( (
    //     match optional1 { Some(v) => size_of::<u8>(),None => 0} + 
    //     match optional2 { Some(v) => size_of::<u16>(),None => 0}
    // ) as u16 ))]
    // #[byteserde(deplete( packet_length as usize - packet_type.len() ))]
    field1: u8,
    overal_length: u16,
    // optional1: Option<u8>,
    // optional2: Option<u16>,
}

impl Default for OptionalNumeric {
    fn default() -> Self {
        Self {
            overal_length: Default::default(),
            // optional1: None,
            // optional2: None,
        }
    }
}

#[test]
fn test_debug() {
    all()
}
fn all() {
    setup::log::configure();
    let inp_debug = OptionalNumeric::default();
    info!("inp_debug: {:?}", inp_debug);

    let tail = &[0x01, 0x02, 0x3];
    // stack
    let mut ser_stack: ByteSerializerStack<135> = to_serializer_stack(&inp_debug).unwrap();
    ser_stack.serialize_bytes_slice(tail).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    // let mut ser_heap = to_serializer_heap(&inp_debug).unwrap();
    // ser_heap.serialize_bytes_slice(tail).unwrap();
    // info!("ser_heap: {ser_heap:#x}");
    // assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // let des = &mut ByteDeserializer::new(ser_stack.as_slice());

    // let out_debug = DebugMsg::byte_deserialize(des).unwrap();
    // info!("out_debug: {:?}", out_debug);
    // info!("des: {:#x}", des);

    // assert_eq!(inp_debug.packet_length + 11, out_debug.packet_length);
    // assert_eq!(inp_debug.packet_type, out_debug.packet_type);
    // assert_eq!(inp_debug.text, out_debug.text);
    // assert_eq!(des.remaining(), tail.len());
}

fn main() {
    all()
}

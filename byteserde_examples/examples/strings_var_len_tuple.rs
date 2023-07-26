mod unittest;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf};
use byteserde_types::{prelude::*, const_char_ascii};
use log::info;
use unittest::setup;

const_char_ascii!(Plus, b'+', ByteSerializeStack, ByteSerializeHeap, ByteSerializedLenOf, PartialEq);

#[derive(ByteDeserializeSlice, ByteSerializeStack, ByteSerializeHeap, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct VariableLenMsg(
    #[byteserde(replace( (_1.byte_len() + _2.byte_len()) as u16 ))] u16,
    Plus,
    #[byteserde(deplete( _0 as usize - _1.byte_len() ))] StringAscii,
);

impl Default for VariableLenMsg {
    fn default() -> Self {
        Self(Default::default(), Default::default(), b"0123456789".into())
    }
}

#[test]
fn test_strings_ascii() {
    strings_ascii()
}

fn strings_ascii() {
    setup::log::configure();
    let inp_debug = VariableLenMsg::default();
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
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    let des = &mut ByteDeserializerSlice::new(ser_stack.as_slice());

    let out_debug = VariableLenMsg::byte_deserialize(des).unwrap();
    info!("out_debug: {:?}", out_debug);
    info!("des: {:#x}", des);

    assert_eq!(inp_debug.0 + 11, out_debug.0);
    assert_eq!(inp_debug.1, out_debug.1);
    assert_eq!(inp_debug.2, out_debug.2);
    assert_eq!(des.remaining(), tail.len());
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, Debug, PartialEq)]
struct Strings(String, char);

#[test]
fn test_strings_utf8() {
    strings_utf8()
}
fn strings_utf8() {
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
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());
    // deserialize
    let out_str: Strings = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_str: {:?}", inp_str);
    info!("out_str: {:?}", out_str);
    assert_eq!(inp_str, out_str);
}
fn main() {
    strings_ascii();
    strings_utf8();
}

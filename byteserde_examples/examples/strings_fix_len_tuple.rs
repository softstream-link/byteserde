mod unittest;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf};
use byteserde_types::{char_ascii, const_char_ascii, prelude::*, string_ascii_fixed};
use log::info;
use unittest::setup;

pub use models::*;
#[rustfmt::skip]
pub mod models{
    use super::*;
    string_ascii_fixed!(UsernameAscii, 10, b' ', true,  #[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq)]);
    char_ascii!(AnyCharAscii, true, #[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq)]);
    const_char_ascii!(XConstCharAscii, b'X', #[derive(ByteSerializeStack, ByteSerializeHeap, PartialEq)]);
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, Debug, PartialEq)]
struct AsciiStrings(
    UsernameAscii,
    #[byteserde(replace(AnyCharAscii::from(b'R')))] AnyCharAscii,
    XConstCharAscii,
    #[byteserde(deplete( _0.byte_len() ))] StringAscii,
);

#[test]
fn test_ascii() {
    ascii()
}
fn ascii() {
    setup::log::configure();

    let inp_str = AsciiStrings(b"will be cut short".as_slice().into(), b'?'.into(), Default::default(), b"my length same as username".into());

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
    info!("ser_stack: {ser_stack:#x}");
    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_str).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());
    // deserialize
    let out_str: AsciiStrings = from_serializer_heap(&ser_heap).unwrap();
    info!("inp_str: {:?}", inp_str);
    info!("out_str: {:?}", out_str);
    assert_eq!(out_str, AsciiStrings(inp_str.0, AnyCharAscii::from(b'R'), inp_str.2, inp_str.3.bytes()[0..10].into(),));
}

fn main() {
    ascii();
}

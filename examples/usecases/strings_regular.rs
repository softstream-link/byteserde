use byteserde::utils::strings::ascii::{CharAscii, ConstCharAscii, StringAscii, StringAsciiFixed};
use byteserde::prelude::*;

type UsernameAscii = StringAsciiFixed<10, b' ', true>;
type AnyCharAscii = CharAscii;
type XCharAscii = ConstCharAscii<b'X'>;

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
struct AsciiStrings {
    username: UsernameAscii,
    #[byteserde(replace( AnyCharAscii::from(b'R') ))]
    anychar: AnyCharAscii,
    always_char_x: XCharAscii,
    #[byteserde(deplete( username.len() ))]
    length_match_username: StringAscii,
}

#[test]
fn test_ascii(){
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();

    let inp_str = AsciiStrings {
        username: b"will be cut short".as_slice().into(),
        anychar: b'?'.into(),
        always_char_x: Default::default(),
        length_match_username: b"my length same as username".into(),
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
    info!("ser_stack: {ser_stack:#x}");
    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_str).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());
    // deserialize
    let out_str: AsciiStrings = from_serializer_heap(&ser_heap).unwrap();
    info!("inp_str: {:?}", inp_str);
    info!("out_str: {:?}", out_str);
    assert_eq!(out_str, AsciiStrings{
        length_match_username: inp_str.length_match_username.bytes()[0..10].into(),
        anychar: AnyCharAscii::from(b'R'),
        ..inp_str
    });
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
struct Strings {
    field_string: String,
    field_char: char,
}

#[test]
fn test_strings() {
    use crate::unittest::setup;
    use log::info;
    setup::log::configure();

    let inp_str = Strings {
        field_string: "whatever".to_string(),
        field_char: '♥', // 3 bytes long
    };
    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
    info!("ser_stack: {ser_stack:#x}");
    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_str).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());
    // deserialize
    let out_str: Strings = from_serializer_heap(&ser_heap).unwrap();
    info!("inp_str: {:?}", inp_str);
    info!("inp_str: {:?}", out_str);
    assert_eq!(inp_str, out_str);
}

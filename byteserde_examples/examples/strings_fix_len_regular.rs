mod unittest;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf};
use byteserde_types::{char_ascii, const_char_ascii, prelude::*, string_ascii_fixed};
use log::info;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Error};
use unittest::setup;

pub use models::*;
#[rustfmt::skip]
pub mod models{
    use super::*;
    string_ascii_fixed!(UsernameAscii, 10, b' ', true, true, #[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq)]);
    char_ascii!(AnyCharAscii, true, #[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq)]);
    const_char_ascii!(XConstCharAscii, b'X', true, #[derive(ByteSerializeStack, ByteSerializeHeap, PartialEq)]);
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, Debug, PartialEq, Serialize, Deserialize)]
// #[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, Debug, PartialEq)]
struct AsciiStrings {
    username: UsernameAscii,
    #[byteserde(replace(AnyCharAscii::from(b'R')))]
    anychar: AnyCharAscii,
    always_char_x: XConstCharAscii,
    #[byteserde(deplete( username.byte_len() ))]
    length_match_username: StringAscii,
}

#[test]
fn test_ascii() {
    ascii_2_bytes();
    ascii_2_json();
}

fn ascii_2_bytes() {
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
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());
    // deserialize
    let out_str: AsciiStrings = from_serializer_heap(&ser_heap).unwrap();
    info!("inp_str: {:?}", inp_str);
    info!("out_str: {:?}", out_str);
    assert_eq!(
        out_str,
        AsciiStrings {
            length_match_username: inp_str.length_match_username.bytes()[0..10].into(),
            anychar: AnyCharAscii::from(b'R'),
            ..inp_str
        }
    );
}

fn ascii_2_json() {
    setup::log::configure();

    let inp_str = AsciiStrings {
        username: b"will be cut short".as_slice().into(),
        anychar: b'?'.into(),
        always_char_x: Default::default(),
        length_match_username: b"my length same as username".into(),
    };

    let jsn_out = to_string(&inp_str).unwrap();
    info!("jsn_out: {}", jsn_out);
    assert_eq!(
        jsn_out,
        r#"{"username":"will be cu","anychar":"?","always_char_x":"X","length_match_username":"my length same as username"}"#
    );
    let out_str: AsciiStrings = from_str(&jsn_out).unwrap();
    info!("out_str: {:?}", out_str);
    assert_eq!(out_str, inp_str);

    // StringAsciiFixed
    let out_err: Result<UsernameAscii, Error> = from_str(r#" "will be cu+overflow" "#);
    info!("out_str: {:?}", out_err);
    assert!(out_err.is_err());
    assert_eq!(
        out_err.unwrap_err().to_string(),
        r#"UsernameAscii being constructed from 'will be cu+overflow' whose byte length: 19 exceeds max allowed byte length: 10 of the tuple struct"#
    );

    // CharAscii
    let out_err: Result<AnyCharAscii, Error> = from_str(r#" "12" "#);
    info!("out_str: {:?}", out_err);
    assert!(out_err.is_err());
    assert_eq!(
        out_err.unwrap_err().to_string(),
        r#"AnyCharAscii being constructed from '12' whose byte length: 2 exceeds max allowed byte length: 1 of the tuple struct"#
    );

    // ConstCharAscii
    let out_err: Result<XConstCharAscii, Error> = from_str(r#" "12" "#);
    info!("out_str: {:?}", out_err);
    assert!(out_err.is_err());
    assert_eq!(
        out_err.unwrap_err().to_string(),
        r#"XConstCharAscii being constructed from '12' whose value does not match expected const: 'X' of the tuple struct"#
    );
}

fn main() {
    ascii_2_bytes();
    ascii_2_json();
}

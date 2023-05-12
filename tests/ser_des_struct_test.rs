mod integrationtest;
use crate::integrationtest::setup;
use byteserde_derive::{ByteSerializeStack, ByteDeserialize};
use byteserde::{
    des::{from_bytes, from_serializer_stack},
    ser::{to_bytes_stack, to_serializer_stack, ByteSerializerStack},
    utils::hex::to_hex_pretty,
};
use log::info;

#[derive(Debug, PartialEq, ByteSerializeStack, ByteDeserialize, Default)]
pub struct Header {
    type_i8: i8,
    type_i16: i16,
}

#[derive(Debug, PartialEq, ByteSerializeStack, ByteDeserialize, Default)]
pub struct Footer {
    type_f32: f32,
    type_f64: f64,
}
#[derive(Debug, PartialEq, ByteSerializeStack, ByteDeserialize, Default)]
pub struct Body {
    type_header: Header,
    type_u8: u8,
    type_i8: i8,
    type_u16: u16,
    type_i16: i16,
    type_footer: Footer,
}

fn get_body() -> Body{
   Body{
    type_header: Header{
        type_i8: -1,
        type_i16: -2,
    },
    type_u8: 1,
    type_i8: 2,
    type_u16: 3,
    type_i16: 4,
    type_footer: Footer{
        type_f32: -1.2,
        type_f64: -2.3,
    },
   }
}
#[test]
fn test_to_from_bytes() {
    setup::log::configure();
    let inp = get_body();
    info!("inp: {inp:?}");
    let bytes: [u8; 128] = to_bytes_stack(&inp).unwrap();
    info!("bytes:\n{}", to_hex_pretty(&bytes));
    let out = from_bytes::<Body>(&bytes).unwrap();
    info!("out: {out:?}");
    assert_eq!(inp, out);
}
#[test]
fn test_structure_to_from_serializer() {
    setup::log::configure();
    let inp = get_body();
    info!("inp: {inp:?}");
    let ser: ByteSerializerStack<128> = to_serializer_stack(&inp).unwrap();
    info!("ser: {ser:#x}");
    let out = from_serializer_stack(&ser).unwrap();
    info!("out: {out:?}");
    assert_eq!(inp, out);
}

use std::mem::size_of;

use crate::integrationtest::setup;
use byteserde::prelude::*;
use log::info;

#[test]
fn test_serialize_string() {
    setup::log::configure();

    let size = size_of::<usize>();
    let ser = &mut ByteSerializerStack::<128>::default();

    let inp = "whatever".to_string();
    let _ = inp.byte_serialize_stack(ser);
    info!("ser: {ser:#x}");

    assert_eq!(
        8_usize.to_be_bytes(),
        ser.as_slice()[0..size]
    );
    assert_eq!(inp.len(), ser.as_slice()[size..].len());
}

#[test]
fn test_serialize_char() {
    setup::log::configure();
    let ser = &mut ByteSerializerStack::<128>::default();

    let inp = 'a';
    let _ = inp.byte_serialize_stack(ser);
    info!("ser: {ser:#x}");

    assert_eq!([0x01_u8], ser.as_slice()[0..1]);
    assert_eq!([0x61_u8], ser.as_slice()[1..2]);
    assert_eq!(ser.len(), 2);
}

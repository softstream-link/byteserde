mod integrationtest;
use crate::integrationtest::setup;
use byteserde::prelude::*;
use log::info;

#[test]
fn test_serialize_string() {
    setup::log::configure();

    let ser = &mut ByteSerializerStack::<128>::default();

    let inp = "whatever".to_string();
    let _ = inp.byte_serialize_stack(ser);
    info!("ser: {ser:#x}");

    // len of string is echoded as u32, ensure it is set to 00 00 00 08 for "whatever" of 8 char
    assert_eq!([0x00_u8, 0x00, 0x00, 0x08], ser.bytes()[0..4]);
    assert_eq!(inp.len(), ser.bytes()[4..].len());
}

#[test]
fn test_serialize_string_too_large() {
    setup::log::configure();

    let ser = &mut ByteSerializerStack::<128>::default();

    let inp = "a".repeat(u32::MAX as usize + 1);
    let out = inp.byte_serialize_stack(ser);
    info!("{out:?}");
    assert!(out.is_err());
    assert_eq!(
        out.unwrap_err().message,
        "max string len supported is 4294967295, but enchountered 4294967296"
    )
}

#[test]
fn test_serialize_char() {
    setup::log::configure();
    let ser = &mut ByteSerializerStack::<128>::default();

    let inp = 'a';
    let _ = inp.byte_serialize_stack(ser);
    info!("ser: {ser:#x}");

    assert_eq!([0x01_u8], ser.bytes()[0..1]);
    assert_eq!([0x61_u8], ser.bytes()[1..2]);
    assert_eq!(ser.len(), 2);
}

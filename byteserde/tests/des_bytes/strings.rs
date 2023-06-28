use crate::integrationtest::setup;
use byteserde::prelude::*;
use log::info;

#[test]
fn test_deserialize_string() {
    setup::log::configure();

    let ser = &mut ByteSerializerStack::<128>::default();

    let inp = "whatever".to_string();
    let _ = inp.byte_serialize_stack(ser);
    info!("ser: {ser:#x}");

    let mut des = ByteDeserializerBytes::new(ser.as_slice().to_vec().into());
    let out: String = des.deserialize().unwrap();
    info!("des: {des:#x}");
    info!("out: {out}");
    assert_eq!(inp, out);

    let ser = &mut ByteSerializerStack::<128>::default();
    let inp = "💖".repeat(8).to_string();
    let _ = inp.byte_serialize_stack(ser);
    info!("ser: {ser:#x}");

    let mut des = ByteDeserializerBytes::new(ser.as_slice().to_vec().into());
    let out: String = des.deserialize().unwrap();
    info!("des: {des:#x}");
    info!("out: {out}");
    assert_eq!(inp, out);
}

#[test]
fn test_serialize_string_too_short_and_not_utf8() {
    setup::log::configure();

    //  create string shorter then its len indicates
    let ser = &mut ByteSerializerStack::<128>::default();
    let _ = ser.serialize_bytes_slice(&8_usize.to_be_bytes());
    let _ = ser.serialize_bytes_slice(&[0xFF_u8]);
    info!("ser: {ser:#x}");
    let mut des = ByteDeserializerBytes::new(ser.as_slice().to_vec().into());
    let out = des.deserialize::<String>();
    info!("{out:?}");
    assert!(out.is_err());

    //  create invalid utf8
    let ser = &mut ByteSerializerStack::<128>::default();
    let _ = ser.serialize_bytes_slice(&8_usize.to_be_bytes());
    let _ = ser.serialize_bytes_slice(&[
        0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8,
    ]);
    info!("ser: {ser:#x}");
    let mut des = ByteDeserializerBytes::new(ser.as_slice().to_vec().into());
    let out = des.deserialize::<String>();
    info!("{out:?}");
    assert_eq!(
        out.unwrap_err().message,
        "bytes slice is not a valid utf8 string bytes: 0000: ff ff ff ff  ff ff ff ff | ÿ ÿ ÿ ÿ  ÿ ÿ ÿ ÿ "
    )
}

#[test]
fn test_deserialize_char() {
    setup::log::configure();
    let ser = &mut ByteSerializerStack::<128>::default();

    let inp = 'a';
    let _ = inp.byte_serialize_stack(ser);
    info!("ser: {ser:#x}");

    let mut des = ByteDeserializerBytes::new(ser.as_slice().to_vec().into());
    let out: char = des.deserialize().unwrap();
    info!("des: {des:#x}");
    info!("out: {out}");
    assert_eq!(inp, out);
}

#[test]
fn test_deserialize_char_too_long_and_not_utf8() {
    setup::log::configure();

    // create invalid len chat
    let ser = &mut ByteSerializerStack::<128>::default();
    let _ = ser.serialize_bytes_slice(&[0x05_u8]);
    info!("ser: {ser:#x}");

    let mut des = ByteDeserializerBytes::new(ser.as_slice().to_vec().into());
    let out = des.deserialize::<char>();
    info!("des: {des:#x}");
    info!("out: {out:?}");
    assert!(out.is_err());
    assert_eq!(
        out.unwrap_err().message,
        "max char len supported 4 but enchountered 5"
    );

    // create invalid utf8
    let ser = &mut ByteSerializerStack::<128>::default();
    let _ = ser.serialize_bytes_slice(&[0x01_u8, 0xFF_u8]);
    info!("ser: {ser:#x}");

    let mut des = ByteDeserializerBytes::new(ser.as_slice().to_vec().into());
    let out = des.deserialize::<char>();
    info!("des: {des:#x}");
    info!("out: {out:?}");
    assert!(out.is_err());
    assert_eq!(
        out.unwrap_err().message,
        "byte slice is not a valid utf8 char. bytes: 0000: ff  | ÿ "
    );
}

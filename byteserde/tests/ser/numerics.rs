// mod integrationtest;
use crate::integrationtest::setup;
use byteserde::prelude::*;
use log::info;

#[test]
fn test_serializer_u16() {
    setup::log::configure();
    fn assert<const CAP: usize>(ser: &mut ByteSerializerStack<CAP>, inp: u16, idx: usize, le: bool) {
        match le {
            true => ser.serialize_le(inp).unwrap(),
            false => ser.serialize_be(inp).unwrap(),
        };
        info!("ser:x {ser:x}");
        info!("idx {idx}");
        info!("inp: {inp}, ipn:x {inp:#06x}, inp:b {inp:016b}");
        println!("{:?}", &ser.as_slice());
        println!("{:?}", &ser.as_slice()[idx..idx + 2]);
        let out = match le {
            true => u16::from_le_bytes(ser.as_slice()[idx..idx + 2].try_into().unwrap()),
            false => u16::from_be_bytes(ser.as_slice()[idx..idx + 2].try_into().unwrap()),
        };
        info!("out: {out}, out:x {out:#06x}, out:b {inp:016b}");
        assert_eq!(inp, out);
        assert_eq!(ser.len(), idx + 2);
    }
    let ser = &mut ByteSerializerStack::<10>::default();
    // as "Little Endian"
    assert(ser, u16::MAX, 0, true);
    assert(ser, 0x000A_u16, 2, true);
    // // as "Big Endian"
    assert(ser, u16::MIN, 4, false);
    assert(ser, 0x000B_u16, 6, false);
}

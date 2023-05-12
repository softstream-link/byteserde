mod integrationtest;
use crate::integrationtest::setup;
use byteserde::{des::ByteDeserializer, error::Result, ser::ByteSerializerStack};
use log::info;

#[test]
fn test_deserializer_u16() {
    setup::log::configure();
    fn assert(inps: &Vec<u16>, le: bool) {
        let ser = &mut ByteSerializerStack::<9>::default();
        for n in inps {
            match le {
                true => ser.serialize_le(*n).unwrap(),
                false => ser.serialize_be(*n).unwrap(),
            };
        }
        // throw in extra byte to make sure last read of u16 fails
        ser.serialize_be(0xff_u8).unwrap();
        info!("ser:x {ser:x}");

        let mut de = ByteDeserializer::new(ser.bytes());
        for inp in inps {
            info!("de:x {de:x}");
            info!("inp: {inp}, ipn:x {inp:#06x}, inp:b {inp:016b}");
            let out: u16 = match le {
                true => de.deserialize_le().unwrap(),
                false => de.deserialize_be().unwrap(),
            };
            info!("out: {out}, out:x {out:#06x}, out:b {out:016b}");
            assert_eq!(*inp, out);
        }
        info!("de:x {de:x}");

        let r: Result<u16> = de.deserialize_le();
        info!("r:? {r:?}");
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().message,
            "buffer len: 9, idx: 8, bytes avail: 1, bytes requested: 2"
        );
    }

    let inps = vec![0x00AA_u16, 0x00BB_u16, 0x00CC_u16, 0x00DD_u16];
    assert(&inps, true);
    assert(&inps, false);
}

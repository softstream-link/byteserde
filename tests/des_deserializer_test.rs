mod integrationtest;
use crate::integrationtest::setup;
use byteserde::prelude::*;
use log::info;

#[test]
#[allow(non_snake_case)]
fn test_Derialser() {
    setup::log::configure();

    // pack a buffer with 20 bytes of payload
    const CAP_22: usize = 22;
    let mut ser = ByteSerializerStack::<CAP_22>::default();
    let inp = &[1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ser = ser
        .serialize_bytes(inp)
        .unwrap()
        .serialize_bytes(inp)
        .unwrap();
    info!("ser: {ser:#x}");

    // make sure you can read 20 bytes of payload
    let mut des = ByteDeserializer::new(ser.bytes());
    info!("des: {des:#x}");
    assert_eq!(des.len(), CAP_22 - 2);
    for _ in 0..2 {
        let out: [u8; 10] = des.deserialize_bytes_array().unwrap();
        info!("inp: {inp:?}, out: {out:?}");
        info!("des: {des:#x}");
    }

    let res_err = des.deserialize_bytes_array::<5>();
    info!("res_err: {res_err:#?}");
    assert!(res_err.is_err());
}

#[test]
fn test_to_serializer() {
    setup::log::configure();
    struct Integers {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: u128,
    }
    impl ByteSerializeStack for Integers {
        fn byte_serialize_stack<const CAP: usize>(
            &self,
            serializer: &mut ByteSerializerStack<CAP>,
        ) -> Result<()> {
            serializer
                .serialize_be(self.a)?
                .serialize_be(self.b)?
                .serialize_be(self.c)?
                .serialize_be(self.d)?
                .serialize_be(self.e)?;
            Ok(())
        }
    }
    let _x = Integers {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
    };

    const JUST_LONG_ENOUGH: usize = 31;
    let ser: ByteSerializerStack<JUST_LONG_ENOUGH> = to_serializer_stack(&_x).unwrap();
    info!("ser {ser:#x}");

    // let des = Deserializer
}

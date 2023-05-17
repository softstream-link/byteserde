use crate::integrationtest::setup;
use byteserde::prelude::*;
use log::info;

#[test]
fn test_serializer_stack_primitive() {
    setup::log::configure();

    // make sure bytes gives you only used part of buffer but capacity still remaining
    const CAP_3: usize = 3;
    let mut ser = ByteSerializerStack::<CAP_3>::default();
    let _ = ser.serialize_bytes_slice(&[1_u8, 2]);
    info!("ser: {ser:#x}");

    assert_eq!(ser.bytes().len(), 2);
    assert_eq!(ser.bytes(), &[1_u8, 2]);
    assert_eq!(ser.capacity(), CAP_3);

    // make sure can write using chained method and can't write past capacity
    const CAP_22: usize = 22;
    let mut ser = ByteSerializerStack::<CAP_22>::default();
    info!("ser: {ser:#x}");
    assert_eq!(ser.len(), 0);
    assert_eq!(ser.capacity(), CAP_22);

    let inp = &[1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let ser = ser
        .serialize_bytes_slice(inp)
        .unwrap()
        .serialize_bytes_slice(inp)
        .unwrap();
    info!("ser: {ser:#x}");
    assert_eq!(ser.len(), CAP_22 - 2);
    assert_eq!(ser.avail(), 2);

    let res_err = ser.serialize_bytes_slice(inp);

    info!("res_err: {res_err:#?}");
    assert!(res_err.is_err());
    assert_eq!(ser.len(), CAP_22 - 2);
    assert_eq!(ser.avail(), 2);
}

#[test]
fn test_serializer_stack_chained_impl() {
    setup::log::configure();
    struct Numerics {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: u128,
    }
    impl ByteSerializeStack for Numerics {
        fn byte_serialize_stack<const CAP: usize>(
            &self,
            serializer: &mut ByteSerializerStack<CAP>,
        ) -> Result<()> {
            serializer
                .serialize_bytes_slice(&[self.a])?
                .serialize_be(self.b)?
                .serialize_be(self.c)?
                .serialize_be(self.d)?
                .serialize_be(self.e)?;
            Ok(())
        }
    }
    let x = Numerics {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
    };
    const TOO_SHORT: usize = 1;
    let err = to_serializer_stack::<TOO_SHORT, Numerics>(&x);
    info!("err {err:#?}");
    assert!(err.is_err());

    const JUST_LONG_ENOUGH: usize = 31;
    let ser: ByteSerializerStack<JUST_LONG_ENOUGH> = to_serializer_stack(&x).unwrap();
    info!("ser {ser:#x}");
    assert_eq!(ser.len(), ser.capacity());
    assert_eq!(ser.len(), JUST_LONG_ENOUGH);
}

use crate::integrationtest::setup;
use byteserde::prelude::*;
use log::info;

#[test]
fn test_derialser() {
    setup::log::configure();

    let bytes = &[
        1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];
    info!("inp: {bytes:?}");
    // make sure you can read 20 bytes of payload
    let mut des = ByteDeserializer::new(bytes);
    info!("des: {des:#x}");
    assert_eq!(des.len(), bytes.len());
    for _ in 0..2 {
        let out: &[u8; 10] = des.deserialize_bytes_array_ref().unwrap();
        info!("out: {out:?}");
        info!("des: {des:#x}");
    }

    let res_err = des.deserialize_bytes_array_ref::<5>();
    info!("res_err: {res_err:#?}");
    assert!(res_err.is_err());
}

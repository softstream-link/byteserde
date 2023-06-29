mod unittest;

use byteserde::prelude::*;
use byteserde_derive::{
    ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf,
};
use log::info;
use unittest::setup;

#[derive(
    ByteSerializeStack,
    ByteSerializeHeap,
    ByteDeserializeSlice,
    ByteSerializedLenOf,
    Debug,
    PartialEq,
    Default,
)]
struct Header(u16);

#[derive(
    ByteSerializeStack,
    ByteSerializeHeap,
    ByteDeserializeSlice,
    ByteSerializedLenOf,
    Debug,
    PartialEq,
)]
struct Variant1 {
    #[byteserde(replace(Header(Variant1::tag())))]
    header: Header,
    data: u32,
}
#[rustfmt::skip]
impl Variant1 { fn tag() -> u16 { 1 } }

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf,Debug, PartialEq)]
struct Variant2a {
    #[byteserde(replace(Header(Variant2a::tag())))]
    header: Header,
    data: u64,
}
#[rustfmt::skip]
impl Variant2a { fn tag() -> u16 { 2 } }

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf,Debug, PartialEq)]
struct Variant2b {
    data: u128,
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf,Debug, PartialEq)]
struct Variant3 {
    #[byteserde(replace(Header(Variant3::tag())))]
    header: Header,
    data: u128,
}
#[rustfmt::skip]
impl Variant3 { fn tag() -> u16 { 3 } }

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf,Debug, PartialEq)]
#[byteserde(peek(0, 2))]
enum Variants {
    #[byteserde(eq(Variant1::tag().to_ne_bytes()))]
    V1(Variant1),
    #[byteserde(eq(Variant2a::tag().to_ne_bytes()))]
    V2(Variant2a, Variant2b),
    // #[byteserde(eq(3_u16.to_ne_bytes()))] // TODO add fail test
    // V3 { x: Variant3 },
}

#[test]
fn test_enum_like_tuple() {
    enum_tuple_like()
}
fn enum_tuple_like() {
    setup::log::configure();

    #[rustfmt::skip]
    let msg_inp = vec![
        Variants::V1( Variant1{header: Header(Variant1::tag()), data: 1}),
        Variants::V2( Variant2a{header: Header(Variant2a::tag()), data: 2}, Variant2b {data: 2},),
    ];

    for msg in &msg_inp {
        info!("msg.byte_len(): {:?}", msg.byte_len());
    }
    let mut iter = msg_inp.iter();
    assert_eq!(iter.next().unwrap().byte_len(), 6);
    assert_eq!(iter.next().unwrap().byte_len(), 26);
    
    let mut ser_stck = ByteSerializerStack::<1024>::default();
    let mut ser_heap = ByteSerializerHeap::default();

    for msg in &msg_inp {
        info!("ser: {:?}", msg);
        let _ = msg.byte_serialize_stack(&mut ser_stck).unwrap();
        let _ = msg.byte_serialize_heap(&mut ser_heap).unwrap();
    }
    info!("ser_stck: {:#x}", ser_stck);
    assert_eq!(ser_stck.as_slice(), ser_heap.as_slice());

    let mut des = ByteDeserializerSlice::new(ser_stck.as_slice());
    let mut msg_out: Vec<Variants> = vec![];
    while !des.is_empty() {
        let msg = des.deserialize::<Variants>().unwrap();
        info!("msg: {:?}", msg);
        msg_out.push(msg);
    }
    assert_eq!(msg_inp, msg_out);
}

fn main() {
    enum_tuple_like();
}

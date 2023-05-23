mod unittest;

use byteserde::prelude::*;
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};
use byteserde_types::char_ascii;
use log::info;
use unittest::setup;

char_ascii!(
    Side,
    ByteSerializeStack,
    ByteSerializeHeap,
    ByteDeserialize,
    Clone
);


#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, PartialEq, Debug)]
#[byteserde(from(Side))]
enum SideEnum {
    Buy,
    Sell,
}
// Used by Serializer
impl From<&SideEnum> for Side {
    fn from(s: &SideEnum) -> Side {
        match s {
            SideEnum::Buy => Side(b'B'),
            SideEnum::Sell => Side(b'S'),
        }
    }
}

// Used by Deserailizer
impl From<Side> for SideEnum {
    fn from(a: Side) -> Self {
        match a {
            Side(b'B') => Self::Buy,
            Side(b'S') => Self::Sell,
            _ => panic!("{:?}, Not mapped to enum", a),
        }
    }
}

#[test]
fn test_enum() {
    enums()
}
fn enums() {
    setup::log::configure();
    let inp_buy = SideEnum::Buy;
    let inp_sel = SideEnum::Sell;
    // SideEnum::from(Side(b'X'));

    info!("inp_buy: {inp_buy:x?}");
    info!("inp_sel: {inp_sel:x?}");
    // stack
    let mut ser_stack = ByteSerializerStack::<128>::default(); // = to_serializer_stack(&inp_side).unwrap();
    ser_stack.serialize(&inp_buy).unwrap();
    ser_stack.serialize(&inp_sel).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let mut ser_heap = ByteSerializerHeap::default();
    ser_heap.serialize(&inp_buy).unwrap();
    ser_heap.serialize(&inp_sel).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    let mut des = ByteDeserializer::new(ser_stack.bytes());
    // deserialize
    let out_buy: SideEnum = des.deserialize().unwrap();
    let out_sel: SideEnum = des.deserialize().unwrap();
    info!("out_buy: {out_buy:x?}");
    info!("out_sel: {out_sel:x?}");
    assert_eq!(inp_buy, out_buy);
    assert_eq!(inp_sel, out_sel);
}

fn main() {
    enums();
}

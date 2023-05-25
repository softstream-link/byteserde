mod unittest;

use byteserde::prelude::*;
use byteserde_derive::{ByteDeserialize, ByteEnumFromBinder, ByteSerializeHeap, ByteSerializeStack};
use byteserde_types::char_ascii;
use log::info;
use unittest::setup;

#[test]
fn test_enums_bind_2_tuple_manual_from() {
    enums_bind_2_tuple_manual_from()
}
fn enums_bind_2_tuple_manual_from() {
    setup::log::configure();
    // create `Side` tuple serializable & deserializable
    char_ascii!(Side, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize);

    // create `SideEnum` enum serializeable & deserializable
    #[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
    // `bind` attribute this will case serializer to use Side::from(&SideEnum)
    #[byteserde(bind(Side))]
    enum SideEnum {
        Buy,
        Sell,
    }
    // `bind` requires this From impl for `Serializer` to work
    impl From<&SideEnum> for Side {
        fn from(v: &SideEnum) -> Self {
            match v {
                SideEnum::Buy => Side(b'B'),
                SideEnum::Sell => Side(b'S'),
            }
        }
    }
    // `bind` requires this From impl for `Deserializer` to work
    impl From<Side> for SideEnum {
        fn from(a: Side) -> Self {
            match a {
                Side(b'B') => Self::Buy,
                Side(b'S') => Self::Sell,
                _ => panic!("{:?}, Not mapped to enum", a),
            }
        }
    }

    let inp_enum_buy = SideEnum::Buy;
    let inp_enum_sel = SideEnum::Sell;

    info!("inp_enum_buy: {:x?}", inp_enum_buy);
    info!("inp_enum_sel: {:x?}", inp_enum_sel);

    // stack
    let mut ser_stack = ByteSerializerStack::<128>::default();
    // serialize enum which is in turn is bound to `Side` struct
    ser_stack.serialize(&inp_enum_buy).unwrap();
    ser_stack.serialize(&inp_enum_sel).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let mut ser_heap = ByteSerializerHeap::default();
    ser_heap.serialize(&inp_enum_buy).unwrap();
    ser_heap.serialize(&inp_enum_sel).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.bytes(), ser_heap.bytes());

    let mut des = ByteDeserializer::new(ser_stack.bytes());
    // deserialize
    let out_enum_buy: SideEnum = des.deserialize().unwrap();
    let out_enum_sel: SideEnum = des.deserialize().unwrap();
    info!("out_enum_buy: {:x?}", out_enum_buy);
    info!("out_enum_sel: {:x?}", out_enum_sel);
    assert_eq!(inp_enum_buy, out_enum_buy);
    assert_eq!(inp_enum_sel, out_enum_sel);
}

#[test]
fn test_enums_from_auto_impl() {
    enums_from_auto_impl()
}
fn enums_from_auto_impl() {
    setup::log::configure();
    
    char_ascii!(Side, PartialEq);

    #[derive(ByteEnumFromBinder, Debug, PartialEq)]
    #[byteserde(bind(Side))]
    #[byteserde(from(&SideEnum))]
    #[byteserde(from(SideEnum))]
    #[byteserde(from(Side))] // will panic on all cases besides covered by replace
    #[byteserde(from(&Side))] // will panic on all cases besides covered by replace
    enum SideEnum {
        #[byteserde(replace(Side(b'B')))]
        Buy,
        #[byteserde(replace(Side(b'S')))]
        Sell,
    }

    // as a result of #[byteserde(from(&SideEnum))]
    let inp_buy: Side = (&SideEnum::Buy).into();
    let inp_sel: Side = (&SideEnum::Sell).into();
    info!("inp_buy: {:x?}", inp_buy);
    info!("inp_sel: {:x?}", inp_sel);
    assert_eq!(inp_buy, Side(b'B'));
    assert_eq!(inp_sel, Side(b'S'));

    // as a result of #[byteserde(from(SideEnum))]
    let inp_buy: Side = SideEnum::Buy.into();
    let inp_sel: Side = SideEnum::Sell.into();
    info!("inp_buy: {:x?}", inp_buy);
    info!("inp_sel: {:x?}", inp_sel);
    assert_eq!(inp_buy, Side(b'B'));
    assert_eq!(inp_sel, Side(b'S'));

    // as a result of #[byteserde(from(Side))]
    let inp_enum_buy: SideEnum = Side(b'B').into();
    let inp_enum_sel: SideEnum = Side(b'S').into();
    info!("inp_enum_buy: {:x?}", inp_enum_buy);
    info!("inp_enum_sel: {:x?}", inp_enum_sel);
    assert_eq!(inp_enum_buy, SideEnum::Buy);
    assert_eq!(inp_enum_sel, SideEnum::Sell);

    // as a result of #[byteserde(from(&Side))]
    let inp_enum_buy: SideEnum = (&Side(b'B')).into();
    let inp_enum_sel: SideEnum = (&Side(b'S')).into();
    info!("inp_enum_buy: {:x?}", inp_enum_buy);
    info!("inp_enum_sel: {:x?}", inp_enum_sel);
    assert_eq!(inp_enum_buy, SideEnum::Buy);
    assert_eq!(inp_enum_sel, SideEnum::Sell);

}

fn main() {
    enums_bind_2_tuple_manual_from();
    enums_from_auto_impl();
}

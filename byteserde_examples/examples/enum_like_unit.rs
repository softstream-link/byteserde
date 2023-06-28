mod unittest;

use byteserde::prelude::*;
use byteserde_derive::{
    ByteDeserializeSlice, ByteEnumFromBinder, ByteSerializeHeap, ByteSerializeStack,
};
use byteserde_types::char_ascii;
use log::info;
use unittest::setup;

#[test]
fn test_enums_from_auto_impl() {
    enums_from_auto_impl()
}
fn enums_from_auto_impl() {
    setup::log::configure();

    #[rustfmt::skip]
    char_ascii!(Side, ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, PartialEq);
    // impl Side{ // TODO deside what todo with FromBInder
    //     pub fn Buy() -> Self { Self(b'B') }
    //     pub fn Sell() -> Self { Self(b'S') }
    // }
    #[rustfmt::skip]
    #[derive(ByteEnumFromBinder, Debug, PartialEq,)]
    #[byteserde(bind(Side))]
    #[byteserde(from(&SideEnum))] // REQUIRED to serialize `SideEnum` as `Side`
    #[byteserde(from(Side))] // REQUIRED to deserialize `Side`  as `SideEnum` impl manually to avoid panic on Sides not in the replace attribute
    #[byteserde(from(SideEnum))] // NOT required just example that it is possible
    // #[byteserde(from(&Side))] // NOT required just example, that it is possible impl manually to avoid panic on Sides not in the replace attribute
    enum SideEnum {
        #[byteserde(replace(Side(b'B')))]
        Buy,
        #[byteserde(replace(Side(b'S')))]
        Sell,
    }

    // stack
    let ser_stack = ByteSerializerStack::<128>::default();
    // // let _ = ser_stack.serialize(&Side::Buy()).unwrap();
    // // let _ = ser_stack.serialize(&SideEnum::Sell.into()).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    // let mut ser_heap = ByteSerializerHeap::default();
    // let _ = ser_heap.serialize(&SideEnum::Buy.into()).unwrap();
    // let _ = ser_heap.serialize(&SideEnum::Sell.into()).unwrap();
    // info!("ser_heap: {ser_heap:#x}");
    // assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    // let mut des = ByteDeserializer::new(ser_stack.as_slice());
    // let out_enum_buy: SideEnum = des.deserialize().unwrap();
    // let out_enum_sel: SideEnum = des.deserialize().unwrap();
    // info!("out_enum_buy: {:?}", out_enum_buy);
    // info!("out_enum_sel: {:?}", out_enum_sel);

    // assert_eq!(out_enum_buy, SideEnum::Buy);
    // assert_eq!(out_enum_sel, SideEnum::Sell);

    // // as a result of #[byteserde(from(&SideEnum))]
    // let inp_buy: Side = (&SideEnum::Buy).into();
    // let inp_sel: Side = (&SideEnum::Sell).into();
    // info!("inp_buy: {:x?}", inp_buy);
    // info!("inp_sel: {:x?}", inp_sel);
    // assert_eq!(inp_buy, Side(b'B'));
    // assert_eq!(inp_sel, Side(b'S'));

    // // as a result of #[byteserde(from(SideEnum))]
    // let inp_buy: Side = SideEnum::Buy.into();
    // let inp_sel: Side = SideEnum::Sell.into();
    // info!("inp_buy: {:x?}", inp_buy);
    // info!("inp_sel: {:x?}", inp_sel);
    // assert_eq!(inp_buy, Side(b'B'));
    // assert_eq!(inp_sel, Side(b'S'));

    // // as a result of #[byteserde(from(Side))]
    // let inp_enum_buy: SideEnum = Side(b'B').into();
    // let inp_enum_sel: SideEnum = Side(b'S').into();
    // info!("inp_enum_buy: {:x?}", inp_enum_buy);
    // info!("inp_enum_sel: {:x?}", inp_enum_sel);
    // assert_eq!(inp_enum_buy, SideEnum::Buy);
    // assert_eq!(inp_enum_sel, SideEnum::Sell);

    // // as a result of #[byteserde(from(&Side))]
    // let inp_enum_buy: SideEnum = (&Side(b'B')).into();
    // let inp_enum_sel: SideEnum = (&Side(b'S')).into();
    // info!("inp_enum_buy: {:x?}", inp_enum_buy);
    // info!("inp_enum_sel: {:x?}", inp_enum_sel);
    // assert_eq!(inp_enum_buy, SideEnum::Buy);
    // assert_eq!(inp_enum_sel, SideEnum::Sell);
}

fn main() {
    enums_from_auto_impl();
}

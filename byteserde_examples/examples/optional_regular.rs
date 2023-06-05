mod unittest;

use byteserde::prelude::*;
use byteserde_derive::{
    ByteDeserialize, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf,
    ByteSerializedSizeOf,
};

use log::info;
use unittest::setup;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct Opt1(#[byteserde(replace( Opt1::tag() ))] u16, u16);
impl Opt1 {
    fn tag() -> u16 {
        1
    }
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct Opt2(#[byteserde(replace( Opt2::tag() ))] u16, u32);
impl Opt2 {
    fn tag() -> u16 {
        2
    }
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct SomeStruct {
    anything_header: i8,
    #[byteserde(replace( optional_section.byte_len() ))]
    optional_section_length: u16,
    #[byteserde(deplete( optional_section_length ))]
    optional_section: OptionalSection,
    anything_foter: i8,
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf,
    Debug, PartialEq)] 
#[byteserde(peek(0, 2))]
struct OptionalSection {
    #[byteserde(eq( Opt1::tag().to_be_bytes() ))]
    optional1: Option<Opt1>,
    #[byteserde(eq( Opt2::tag().to_be_bytes() ))]
    optional2: Option<Opt2>,
}

impl Default for OptionalSection {
    fn default() -> Self {
        Self {
            optional1: None,
            optional2: Some(Opt2(Opt2::tag(), 2_u32)),
        }
    }
}
impl Default for SomeStruct {
    fn default() -> Self {
        Self {
            anything_header: -1,
            optional_section_length: 0,
            optional_section: Default::default(),
            anything_foter: -1,
        }
    }
}

#[test]
fn test_optional_block() {
    optional_block()
}
fn optional_block() {
    setup::log::configure();
    let sz_of_opt1 = Option::<Opt1>::byte_size();
    let sz_of_opt2 = Option::<Opt2>::byte_size();
    info!("sz_of_opt1: {:?}", sz_of_opt1);
    info!("sz_of_opt2: {:?}", sz_of_opt2);
    assert_eq!(Option::<Opt1>::byte_size(), 4);
    assert_eq!(Option::<Opt2>::byte_size(), 6);

    let inp_opt = SomeStruct::default();
    info!("inp_opt: {:?}", inp_opt);
    let ln_of_inp_opt = inp_opt.optional_section.byte_len();
    let ln_of_opt1 = inp_opt.optional_section.optional1.byte_len();
    let ln_of_opt2 = inp_opt.optional_section.optional2.byte_len();
    info!("ln_of_inp_opt: {:?}", ln_of_inp_opt);
    info!("ln_of_opt1: {:?}", ln_of_opt1);
    info!("ln_of_opt2: {:?}", ln_of_opt2);
    assert_eq!(ln_of_opt1, 0); // defaulted to None
    assert_eq!(ln_of_opt2, 6); // defaulted to Some(Opt2(2, 2_16))
    assert_eq!(ln_of_inp_opt, 6);

    // // stack
    let ser_stack: ByteSerializerStack<135> = to_serializer_stack(&inp_opt).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap = to_serializer_heap(&inp_opt).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    let des = &mut ByteDeserializer::new(ser_stack.as_slice());

    let out_opt = SomeStruct::byte_deserialize(des).unwrap();
    info!("out_opt: {:?}", out_opt);
    info!("des: {:#x}", des);
    assert_eq!(
        out_opt,
        SomeStruct {
            optional_section_length: 6,
            ..inp_opt
        }
    );
}

fn main() {
    optional_block()
}

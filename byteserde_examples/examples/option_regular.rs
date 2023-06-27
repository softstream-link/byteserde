mod unittest;

use byteserde::prelude::*;
use byteserde_derive::{
    ByteDeserialize, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf,
    ByteSerializedSizeOf,
};

use log::info;
use unittest::setup;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf, 
        Debug, PartialEq, Clone)]
#[byteserde(endian = "be")]
struct Opt1(#[byteserde(replace( Opt1::tag() ))] u16, u16);
impl Opt1 {
    fn tag() -> u16 {
        1
    }
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf, 
        Debug, PartialEq, Clone)]
#[byteserde(endian = "be")]
struct Opt2(#[byteserde(replace( Opt2::tag() ))] u16, u32);
impl Opt2 {
    fn tag() -> u16 {
        2
    }
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq, Clone)]
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
    Debug, PartialEq, Clone)] 
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
            optional1: Some(Opt1(Opt1::tag(), 1_u16)),
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

    let inp_opt1 = SomeStruct::default();
    let mut inp_opt2 = SomeStruct::default();
    inp_opt2.optional_section.optional2 = None;
    let mut inp_opt3 = SomeStruct::default();
    inp_opt3.optional_section.optional1 = None;
    let mut inp_opt4 = SomeStruct::default();
    inp_opt4.optional_section.optional1 = None;
    inp_opt4.optional_section.optional2 = None;
    info!("inp_opt1: {:?}", inp_opt1); // some / some
    info!("inp_opt2: {:?}", inp_opt2); // some / none
    info!("inp_opt3: {:?}", inp_opt3); // none / some
    info!("inp_opt4: {:?}", inp_opt4); // none / none
    
    let ln_of_inp_opt = inp_opt1.optional_section.byte_len();
    let ln_of_opt1 = inp_opt1.optional_section.optional1.byte_len();
    let ln_of_opt2 = inp_opt1.optional_section.optional2.byte_len();
    info!("ln_of_inp_opt: {:?}", ln_of_inp_opt);
    info!("ln_of_opt1: {:?}", ln_of_opt1);
    info!("ln_of_opt2: {:?}", ln_of_opt2);
    assert_eq!(ln_of_inp_opt, 10);
    assert_eq!(ln_of_opt1, 4); // defaulted 
    assert_eq!(ln_of_opt2, 6); // defaulted 

    
    // stack
    let mut ser_stack: ByteSerializerStack<135> = to_serializer_stack(&inp_opt1).unwrap();
    // inp_opt.optional_section.optional1 = Some(Opt1(Opt1::tag(), 1_u16));
    ser_stack.serialize(&inp_opt2).unwrap();
    ser_stack.serialize(&inp_opt3).unwrap();
    ser_stack.serialize(&inp_opt4).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let mut ser_heap = to_serializer_heap(&inp_opt1).unwrap();
    ser_heap.serialize(&inp_opt2).unwrap();
    ser_heap.serialize(&inp_opt3).unwrap();
    ser_heap.serialize(&inp_opt4).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    let des = &mut ByteDeserializer::new(ser_stack.as_slice());

    let out_opt1 = SomeStruct::byte_deserialize(des).unwrap();
    let out_opt2 = SomeStruct::byte_deserialize(des).unwrap();
    let out_opt3 = SomeStruct::byte_deserialize(des).unwrap();
    let out_opt4 = SomeStruct::byte_deserialize(des).unwrap();
    info!("out_opt1: {:?}", out_opt1);
    info!("out_opt2: {:?}", out_opt2);
    info!("out_opt3: {:?}", out_opt3);
    info!("out_opt4: {:?}", out_opt4);
    
    info!("des: {:#x}", des);
    assert_eq!(
        out_opt1,
        SomeStruct {
            optional_section_length: 10,
            ..inp_opt1
        }
    );
    assert_eq!(
        out_opt2,
        SomeStruct {
            optional_section_length: 4,
            ..inp_opt2
        }
    );
    assert_eq!(
        out_opt3,
        SomeStruct {
            optional_section_length: 6,
            ..inp_opt3
        }
    );
    assert_eq!(
        out_opt4,
        SomeStruct {
            optional_section_length: 0,
            ..inp_opt4
        }
    );
}

fn main() {
    optional_block()
}

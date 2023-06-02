mod unittest;

use byteserde::prelude::*;
use byteserde_derive::{
    ByteDeserialize, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf,
    ByteSerializedSizeOf,
};
// use byteserde_derive::{ByteSerializedLenOf};
// use byteserde_types::prelude::*;
use log::info;
use unittest::setup;

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct Opt1(#[byteserde(replace( Opt1::tag() ))] u16, u16);
impl Opt1{
    fn tag() -> u16 { 1 }
}
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct Opt2(#[byteserde(replace( Opt2::tag() ))] u16, u32);
impl Opt2{
    fn tag() -> u16 { 2 }
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct SomeStruct{
    anything_header: i8,

    #[byteserde(replace( body.byte_len() ))]
    body_length: u16,
    #[byteserde(deplete( body_length ))]
    body: OptionalSection,

    anything_foter: i8,
}

#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq)]  // TODO  - ByteSerializedLenOf
#[byteserde(peek(0, 2))]
struct OptionalSection {
    #[byteserde(eq( Opt1::tag() ))]
    optional1: Option<Opt1>,
    #[byteserde(eq( Opt2::tag() ))]
    optional2: Option<Opt2>,
}

// impl ByteDeserialize<OptionalSection> for OptionalSection {
//     fn byte_deserialize(des: &mut ByteDeserializer) -> Result<OptionalSection> {
//         let mut optional1  = None;
//         let mut optional2 = None;
//         while !des.is_empty(){
//             let peek = |start, len| -> Result<&[u8]> {
//                 let p = des.peek_bytes_slice(len + start)?;
//                 Ok(&p[start..])
//             };
//             let __peeked = peek(0, 2)?;
//             if __peeked == 1_u16.to_be_bytes() {
//                 optional1 = Some(des.deserialize()?);
//                 continue;
//             }
//             if __peeked == 2_u16.to_be_bytes() {
//                 optional2 = Some(des.deserialize()?);
//                 continue;
//             }
//         }
//         Ok(OptionalSection{
//             optional1,
//             optional2,
//         })
//     }
// }

impl Default for OptionalSection {
    fn default() -> Self {
        Self {
            // optional1: None,
            optional1: Some(Opt1(Opt1::tag(), 1_u16)),
            optional2: Some(Opt2(Opt1::tag(), 2_u32)),
        }
    }
}
impl Default for SomeStruct{
    fn default() -> Self {
        Self {
            anything_header: -1,
            body_length: 0,
            body: Default::default(),
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
    // assert_eq!(Option::<Opt1>::byte_size(), 1);
    // assert_eq!(Option::<Opt2>::byte_size(), 2);

    let inp_opt = SomeStruct::default();
    info!("inp_opt: {:?}", inp_opt);
    let ln_of_opt1 = inp_opt.body.optional1.byte_len();
    let ln_of_opt2 = inp_opt.body.optional2.byte_len();
    info!("ln_of_opt1: {:?}", ln_of_opt1);
    info!("ln_of_opt2: {:?}", ln_of_opt2);
    // assert_eq!(ln_of_opt1, 0); // defaulted to None
    // assert_eq!(ln_of_opt2, 2); // defaulted to Some(Opt2(2_16))

    // let tail = &[0x01, 0x02, 0x3];
    // // stack
    let ser_stack: ByteSerializerStack<135> = to_serializer_stack(&inp_opt).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let mut ser_heap = to_serializer_heap(&inp_opt).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    let des = &mut ByteDeserializer::new(ser_stack.as_slice());
    
        
    let out_opt = SomeStruct::byte_deserialize(des).unwrap();
    info!("out_opt: {:?}", out_opt);
    info!("des: {:#x}", des);

    // assert_eq!(inp_debug.packet_length + 11, out_debug.packet_length);
    // assert_eq!(inp_debug.packet_type, out_debug.packet_type);
    // assert_eq!(inp_debug.text, out_debug.text);
    // assert_eq!(des.remaining(), tail.len());
}

fn main() {
    optional_block()
}

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

#[derive(ByteSerializeStack, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct Opt1(#[byteserde(replace( Opt1::tag() ))] u8, u16);
impl Opt1{
    fn tag() -> u8 { 1 }
}
#[derive(ByteSerializeStack, ByteDeserialize, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct Opt2(#[byteserde(replace( Opt2::tag() ))] u8, u32);
impl Opt2{
    fn tag() -> u8 { 2 }
}

#[derive(ByteSerializeStack, ByteDeserialize, Debug, PartialEq)]
#[byteserde(endian = "be")]
struct OptionalBlock{
    #[byteserde(replace( body.byte_len() ))]
    overal_length: u16,
    #[byteserde(deplete( overal_length ))]
    body: OptionalSection,
}
#[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq)]  // TODO  - ByteSerializedLenOf
struct OptionalSection {
    optional1: Option<Opt1>,
    optional2: Option<Opt2>,
}

impl ByteDeserialize<OptionalSection> for OptionalSection {
    fn byte_deserialize(des: &mut ByteDeserializer) -> Result<OptionalSection> {
        let peek = |len| des.peek_bytes_slice(len);

        
        let mut optional1  = None;
        let mut optional2 = None;
        while !des.is_empty(){
            let x = des.peek_bytes_slice(1)?;
            let y = peek(1)?;
            if x == &[1]{
                optional1 = Some(des.deserialize()?);
            }
            else if x == &[2]{
                optional2 = Some(des.deserialize()?);
            }
            else{
                continue;
            }
        }
        Ok(OptionalSection{
            optional1,
            optional2,
        })
    }
}

impl Default for OptionalSection {
    fn default() -> Self {
        Self {
            // optional1: None,
            optional1: Some(Opt1(0, 1_u16)),
            optional2: Some(Opt2(0, 2_u32)),
        }
    }
}
impl Default for OptionalBlock{
    fn default() -> Self {
        Self {
            overal_length: 0,
            body: Default::default(),
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

    let inp_opt = OptionalBlock::default();
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
    // let mut ser_heap = to_serializer_heap(&inp_debug).unwrap();
    // ser_heap.serialize_bytes_slice(tail).unwrap();
    // info!("ser_heap: {ser_heap:#x}");
    // assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    let des = &mut ByteDeserializer::new(ser_stack.as_slice());
    
    let out_opt = OptionalBlock::byte_deserialize(des).unwrap();
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

mod unittest;
use byteserde::prelude::*;
use byteserde_derive::{
    ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf,
    ByteSerializedSizeOf,
};
use log::info;
use unittest::setup;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, 
       ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone)]
#[byteserde(endian = "le")]
pub struct NumbersStructRegular<const L: usize, const M: usize> {
    #[byteserde(endian = "be")]
    filed_arr_u16_local_macro: [u16; L],
    filed_arr_u16_global_macro: [u16; M],
}
impl<const L: usize, const M: usize> Default for NumbersStructRegular<L, M> {
    fn default() -> Self {
        Self {
            filed_arr_u16_local_macro: [Default::default(); L],
            filed_arr_u16_global_macro: [Default::default(); M],
        }
    }
}
#[test]
fn test_numeric() {
    numeric()
}
fn numeric() {
    setup::log::configure();
    let inp_num = NumbersStructRegular::<2, 3> {
        filed_arr_u16_local_macro: [0x0001_u16, 0x0002_u16],
        filed_arr_u16_global_macro: [0x0001_u16, 0x0002_u16, 0x0003_u16],
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_num).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_num).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_num: NumbersStructRegular<2, 3> = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_num: {inp_num:?}");
    info!("out_num: {out_num:?}");
    assert_eq!(inp_num, out_num);
}
#[test]
fn test_numeric_size_len() {
    numeric_size_len();
}
fn numeric_size_len(){
    setup::log::configure();
    let inp_num = NumbersStructRegular::<1, 2>::default();
    let sz_of = NumbersStructRegular::<1, 2>::byte_size();
    let ln_of = inp_num.byte_len();
    info!("inp_num: {inp_num:?}");
    info!("sz_of: {sz_of}");
    info!("ln_of: {ln_of}");
    assert_eq!(sz_of, 6);
    assert_eq!(ln_of, sz_of);
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice,
         ByteSerializedLenOf, Debug, PartialEq, Clone)]
pub struct StringsStructRegular<S, C> 
where 
    S: ByteSerializeStack + ByteSerializeHeap + ByteDeserializeSlice<S> + ByteSerializedLenOf,
    C: ByteSerializeStack + ByteSerializeHeap + ByteDeserializeSlice<C> + ByteSerializedLenOf,
{
    field_string: S,
    field_char: C,
}

impl Default for StringsStructRegular<String, char> {
    fn default() -> Self {
        Self {
            field_string: "hello".to_string(),
            field_char: 'h',
        }
    }
}
#[test]
fn test_strings() {
    strings()
}
fn strings() {
    setup::log::configure();
    let inp_str = StringsStructRegular::<String, char> {
        field_string: "Hello".to_string(),
        field_char: 'a',
    };
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_str).unwrap();
    info!("ser_heap: {ser_heap:#x}");

    let out_str = from_serializer_stack(&ser_stack).unwrap();
    info!("inp_str: {inp_str:?}");
    info!("out_str: {out_str:?}");
    assert_eq!(inp_str, out_str);
}
#[test]
fn test_strings_len(){
    strings_len();
}
fn strings_len(){
    setup::log::configure();
    let inp_str = StringsStructRegular::<String, char>{
        field_string: "12345".to_string(),
        field_char: 'a',
    };
    let ln_of = inp_str.byte_len();
    info!("inp_str: {inp_str:?}");
    info!("ln_of: {ln_of}");
    assert_eq!(ln_of, 6);

    let inp_str = StringsStructRegular::<String, char>{
        field_string: "1234567890".to_string(),
        field_char: 'a',
    };
    let ln_of = inp_str.byte_len();
    info!("inp_str: {inp_str:?}");
    info!("ln_of: {ln_of}");
    assert_eq!(ln_of, 11);
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, 
        ByteSerializedLenOf, Debug, PartialEq)]
pub struct NestedStructRegular<
    const L: usize,
    const M: usize,
    S: ByteSerializeStack + ByteSerializeHeap + ByteDeserializeSlice<S> + ByteSerializedLenOf,
    C: ByteSerializeStack + ByteSerializeHeap + ByteDeserializeSlice<C> + ByteSerializedLenOf,
> {
    field_numbers: NumbersStructRegular<L, M>,
    field_strings: StringsStructRegular<S, C>,
}

#[test]
fn test_nested() {
    nested()
}
fn nested() {
    setup::log::configure();
    let inp_nes = NestedStructRegular::<2, 3, String, char> {
        field_numbers: Default::default(),
        field_strings: Default::default(),
    };

    // stack
    let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_nes).unwrap();
    info!("ser_stack: {ser_stack:#x}");

    // heap
    let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_nes).unwrap();
    info!("ser_heap: {ser_heap:#x}");
    assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

    // deserialize
    let out_nes: NestedStructRegular<2, 3, String, char> =
        from_serializer_stack(&ser_stack).unwrap();
    info!("inp_nes: {inp_nes:?}");
    info!("out_nes: {out_nes:?}");
    assert_eq!(inp_nes, out_nes);
}
#[test]
fn test_nested_len(){
    nested_len()
}
fn nested_len(){
    setup::log::configure();
    let inp_nes = NestedStructRegular::<1, 2, String, char>{
        field_numbers: Default::default(), // len => 1 * 2(u16) + 2 * 2(u16) = 6
        field_strings: Default::default(), // len => hello + h = 6
    };
    let ln_of = inp_nes.byte_len();
    info!("inp_nes: {inp_nes:?}");
    info!("ln_of: {ln_of}");
    assert_eq!(ln_of, 12);
}
fn main() {
    numeric();
    numeric_size_len();
    strings();
    strings_len();
    nested();
    nested_len();

}

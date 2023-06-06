#[allow(unused_imports)]
use byteserde::prelude::*;

use byteserde_derive::ByteDeserialize;

#[derive(ByteDeserialize)]
struct TupleStruct(u8, u8);

#[derive(ByteDeserialize)]
struct OptionalSectionAllMustBeStruct {
    field: Option<u8>,
    field1: Option<TupleStruct>,
}

#[derive(ByteDeserialize)]
struct OptionalSectionMissingEqAnnotation {
    field1: Option<TupleStruct>,
}

#[derive(ByteDeserialize)]
struct OptionalSectionMissingPeekAnnotation {
    #[byteserde(eq( 1_u16.to_be_bytes() ))]
    field1: Option<TupleStruct>,
}

#[derive(ByteDeserialize)]
#[byteserde(peek(0, 2))]
struct OptionalSectionAllMustBeOption {
    field: TupleStruct,
    #[byteserde(eq( 1_u16.to_be_bytes() ))]
    field1: Option<TupleStruct>,
}

fn main() {}

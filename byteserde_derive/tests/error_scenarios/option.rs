#[allow(unused_imports)]
use byteserde::prelude::*;

use byteserde_derive::ByteDeserializeSlice;

#[derive(ByteDeserializeSlice)]
struct TupleStruct(u8, u8);

#[derive(ByteDeserializeSlice)]
struct OptionalSectionAllMustBeStruct {
    field: Option<u8>,
    field1: Option<TupleStruct>,
}

#[derive(ByteDeserializeSlice)]
struct OptionalSectionMissingEqAnnotation {
    field1: Option<TupleStruct>,
}

#[derive(ByteDeserializeSlice)]
struct OptionalSectionMissingPeekAnnotation {
    #[byteserde(eq( 1_u16.to_be_bytes() ))]
    field1: Option<TupleStruct>,
}

#[derive(ByteDeserializeSlice)]
#[byteserde(peek(0, 2))]
struct OptionalSectionAllMustBeOption {
    field: TupleStruct,
    #[byteserde(eq( 1_u16.to_be_bytes() ))]
    field1: Option<TupleStruct>,
}

fn main() {}

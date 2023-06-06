use byteserde_derive::ByteSerializedSizeOf;

#[derive(ByteSerializedSizeOf)]
struct VecRegular {
    field: Vec<u8>,
}

#[derive(ByteSerializedSizeOf)]
struct VecTuple(Vec<u8>);

fn main() {}

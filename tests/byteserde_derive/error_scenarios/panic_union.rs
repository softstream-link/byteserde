use byteserde_derive::ByteSerializeStack;

#[derive(ByteSerializeStack)]
union NotSupportedUnion {
    a: i32,
    b: i16,
}

fn main() {}

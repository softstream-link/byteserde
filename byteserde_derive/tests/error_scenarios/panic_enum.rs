use byteserde_derive::ByteSerializeStack;

#[derive(ByteSerializeStack)]
enum NotSupportedEnum {
    NotEvenThis(String),
    OrThis(u8),
}

fn main() {}

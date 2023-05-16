

# Development hints
* When working on or improving the `byteserde` proc macro the following setup is required to run `cargo expand`
    1. Add the following block in the [Cargo.toml](Cargo.toml)
        ```toml
        [[bin]]
        name = "main"
        path = "main.rs"
        ```
    2. create a [main.rs](main.rs) file in the root of the project
        ```rust
        use byteserde::{ByteSerializerStack, ByteDeserializer};
        use byteserde_derive::{ByteSerializeStack, ByteDeserialize};

        #[derive(ByteSerializeStack, ByteDeserialize)]
        pub struct NumbersStructRegular {
            field_i8: i8,
            field_u8: u8,
        }
        fn main() {}
        ```
    3. run expand command
        ```sh
        cargo expand --bin main
        ```
    4. It shall expand into something that looks as following
        ```rust
        ////////// snip ...
        impl byteserde::ser::ByteSerializeStack for NumbersStructRegular {
            fn byte_serialize_stack<const CAP: usize>(
                &self,
                ser: &mut byteserde::ser::ByteSerializerStack<CAP>,
            ) -> byteserde_bin::error::Result<()> {
                ser.serialize_ne(self.field_i8)?;
                ser.serialize_ne(self.field_u8)?;
                Ok(())
            }
        }
        impl byteserde::des::ByteDeserialize<NumbersStructRegular> for NumbersStructRegular {
            fn byte_deserialize(
                des: &mut byteserde_bin::des::ByteDeserializer,
            ) -> byteserde::error::Result<NumbersStructRegular> {
                Ok(NumbersStructRegular {
                    field_i8: des.deserialize_ne()?,
                    field_u8: des.deserialize_ne()?,
                })
            }
        }
        ////////// snip ...
        ```


* All unittests related to the proc macros are defined here  [tests/byteserde/usecases_test.rs](tests/byteserde/usecases_test.rs)

    ```sh
    cargo test --package byteserde_bin --test mod -- byteserde::usecases_test --nocapture
    ```



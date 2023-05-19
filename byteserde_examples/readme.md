# Examples
* The following list of examples is available in the order of incremental complexity.
* All examples are implemented in the form of a test, where the structure of a given complexity is `initialized`, `serialized`, and then `deserialized` with the expectation that starting and resulting `structs` are identical. 
* All examples provide both `stack` & `heap` serializers for refernce. 
    *  NOTE: In each case an example is available in Rust's `regular` & `tuple` `struct` format

## `Numerics` ( `u8`, `u16`, `i32`, ..)
* [Regular](./examples/numeric_regular.rs)
* [Tuple](./examples/numeric_tuple.rs)

## `Strings` `ascii` / `utf-8`
* [Regular](./examples/strings_regular.rs)
* [Tuple](./examples/strings_tuple.rs)
    * `ascii` types are included with the package
        * [StringAsciiFixed](../byteserde_types/src/utils/strings/ascii/mod.rs#StringAsciiFixed) - fixed length string
        * [StringAscii](../byteserde_types/src/utils/strings/ascii/mod.rs#StringAscii) - variable length string
        * [CharAscii](../byteserde_types/src/utils/strings/ascii/mod.rs#CharAscii) - char, one byte long
        * [ConstCharAscii](../byteserde_types/src/utils/strings/ascii/mod.rs#ConstCharAscii) - constant char, one byte long

## `Arrays` of `u8`, `u16`, `i32`, .. / `ascii`, `utf-8` strings / other arbitrary types
* [Regular](./examples/arr_regular.rs)
* [Tuple](./examples/arr_tuple.rs)

## `Vector` of `u8`, `u16`, `i32`, .. / `ascii`, `utf-8` strings / other arbitrary types
* [Regular](./examples/vec_regular.rs)
* [Tuple](./examples/vec_tuple.rs)

## `Generics` support
* [Regular](./examples/generics_regular.rs)
* [Tuple](./examples/generics_tuple.rs)

## `Practical` example of an actual network packet message 
* [Regular](./examples/practical_regular.rs)
* [Tuple](./examples/practical_tuple.rs)
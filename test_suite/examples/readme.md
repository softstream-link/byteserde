# Examples
* The following list of examples is available in the order of incremental complexity.
* All examples are implemented in the form of a test, where the structure of a given complexity is `initialized`, `serialized`, and then `deserialized` with the expectation that starting and resulting `structs` are identical. 
* All examples provide both `stack` & `heap` serializers for refernce. 
    *  NOTE: In each case an example is available in Rust's `regular` & `tuple` `struct` format

## `Numerics` ( `u8`, `u16`, `i32`, ..)
* [Regular](./usecases/numeric_regular.rs)
* [Tuple](./usecases/numeric_tuple.rs)

## `Strings` `ascii` / `utf-8`
* [Regular](./usecases/strings_regular.rs)
* [Tuple](./usecases/strings_tuple.rs)
    * `ascii` types are included with the package
        * [StringAsciiFixed](../src/utils/strings/ascii/mod.rs#StringAsciiFixed) - fixed length string
        * [StringAscii](../src/utils/strings/ascii/mod.rs#StringAscii) - variable length string
        * [CharAscii](../src/utils/strings/ascii/mod.rs#CharAscii) - char, one byte long
        * [ConstCharAscii](../src/utils/strings/ascii/mod.rs#ConstCharAscii) - constant char, one byte long

## `Arrays` of `u8`, `u16`, `i32`, .. / `ascii`, `utf-8` strings / other arbitrary types
* [Regular](./usecases/arr_regular.rs)
* [Tuple](./usecases/arr_tuple.rs)

## `Vector` of `u8`, `u16`, `i32`, .. / `ascii`, `utf-8` strings / other arbitrary types
* [Regular](./usecases/vec_regular.rs)
* [Tuple](./usecases/vec_tuple.rs)

## `Generics` support
* [Regular](./usecases/generics_regular.rs)
* [Tuple](./usecases/generics_tuple.rs)

## `Practical` example of an actual network packet message 
* [Regular](./usecases/practical_regular.rs)
* [Tuple](./usecases/practical_tuple.rs)
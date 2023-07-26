# Examples
* The following list of examples are listed in the order of incremental complexity.
* All examples are implemented in the form of a test, where the structure of a given complexity is `initialized`, `serialized`, and then `deserialized` with the expectation that starting and resulting `structs` are identical, with a few desired exceptions.
* All examples provide both `stack` & `heap` serializers for refernce. 
    

## `Numerics` ( `u8`, `u16`, `i32`, ..)
* Comprehensive Examples & Tests - [regular](numeric_regular.rs) / [Tuple](numeric_tuple.rs)
* Both examples are identical with exception of using a `struct` with `named fields` vs a `tuple`. However, they demonstrate how to use several important `byteserde` attribute features on the `stuct` of each type, namely:
  
  * `#[byteserde(replace( ... ))]` - this is a `field` level attribute and it will only affect `serialization`. `( ... )` expression must evaluate to the same type as the field it annotates and can reference other `struct` members by name. Usefulness of this attribute will be covered in more advanced examples but for now you just need to know that it is possible to ignore instance value and serialize a different value of same time.
    
    * Example: calling `to_serializer_stack( &WithReplace{ value: 9 } )` will produce a byte steream which contains `0x03` instead of `0x09` which is value of the instance
        ```rust
        #[derive(ByteSerializeStack)]
        struct WithReplace{
            #[byteserde(replace( 3 ))]
            value: u8
        }
        ```

  * `#[byteserde(endian = "be" )]` - this attribute affect both `serialization` and `deserialization` and can be used at both `struct` and `field` level. It will affect all `rust` numeric types (signed, unsigned, floating point, and integers). `be`, `le`, `ne` stand for `Big Endian`, `Little Endian`, and `Native Endian` (default) respectively

    * Example: In below calling `let ser = to_serializer_stack( &WithEndian{ be: 1, le: 2} )` will produce a byte stream `0x00 0x01 0x02 0x00` with first pair bytes representing `WithEndian.be` field and second pari of bytes representing `WithEndian.le` field. Note, that this attribute also affects `deserialization`, which is a good thing because it means that both pair of bytes will be correctly interpreted when calling `let x: WithEndian = from_serializer_stack(&ser)`
        ```rust
        #[derive(ByteSerializeStack, ByteDeserializeSlice)]
        #[byteserde(endian = "be")]
        struct WithEndian{
            be: u16,
            #[byteserde(endian = "le")]
            le: u16,
        }
        ```
  
## `Fixed & Variable Length Strings` - `asci` & `utf-8`
* Comprehensive Examples & tests 
  * [Regular](strings_fix_len_regular.rs) / [Tuple](strings_fix_len_tuple.rs) - `fixed length strings/ascii, mostly :)`
  * [Regular](strings_var_len_regular.rs) / [Tuple](strings_var_len_tuple.rs) - `variable length strings/utf-8`

* Just like for numerics both examples are identicalw with exception of using a `struct` with `named fields` vs a `tuple`. This example expands on numerics and introduces one additional `byteserde` attribute, namely:

    * `#[byteserde(deplete( ... ))]` - unlike `replace` this attribute only affects `deserialization` by limiting the number of bytes available to the annotated `struct` member during deserialization. `( ... )` expression need to evaluate to `usize` and can reference other `struct` members by name. This is useful when the protocol has variable length strings whose length is expressed as a value of an other struct member. 
    
      * Example: In below calling `let x: WithDeplete = from_serializer_stack(&ser)` will ensure that a `msg` member cannot see beyond value of the `msg_length` during deserialization, whose value is guaranteed to always be set to `msg.len()` during serialization
          ```rust
          #[derive(ByteSerializeStack, ByteDeserializeSlice)]
          struct WithDeplete{
              #[byteserde(replace( msg.len() ))]
              msg_length: u16,
              #[byteserde(deplete( msg_length ))]
              msg: StringAscii,
          }
          ```
  
    * `ascii` types & macros are included with the `byteserde_types` crate
      * Types
        * [StringAsciiFixed](../../byteserde_types/src/strings/ascii/mod.rs#StringAsciiFixed) - fixed length string
        * [CharAscii](../../byteserde_types/src/strings/ascii/mod.rs#CharAscii) - char, one byte long
        * [ConstCharAscii](../../byteserde_types/src/strings/ascii/mod.rs#ConstCharAscii) - constant char, one byte long
        * [StringAscii](../../byteserde_types/src/strings/ascii/mod.rs#StringAscii) - variable length string using `Vec<u8>` this is a greedy type since it does not know its size at compile time will consume remaining byte stream unless limited by `deplete` attribute
      * Macros
        * [string_ascii_fixed!](../../byteserde_types/src/macros/mod.rs) - generates a `StringAsciiFixed` like type but with preffered name, length, padding and alignment
        * [char_ascii!](../../byteserde_types/src/macros/mod.rs) - generates a `CharAscii` like type but with a preffered name
        * [const_char_ascii!](../../byteserde_types/src/macros/mod.rs) - generates a `ConstcharAscii` like type but with preffered name and default ascii const char.
  * 


## `Arrays` of `u8`, `u16`, `i32`, .. / `ascii`, `utf-8` strings / other arbitrary types
* Comprehensive Examples & tests [Regular](arr_regular.rs) / [Tuple](arr_tuple.rs)

## `Vector` of `u8`, `u16`, `i32`, .. / `ascii`, `utf-8` strings / other arbitrary types
* Comprehensive Examples & tests [Regular](vec_regular.rs) / [Tuple](vec_tuple.rs)

## `Generics` support
* Comprehensive Examples & tests [Regular](generics_regular.rs) / [Tuple](generics_tuple.rs)

## `Option<T>` support
* Comprehensive Examples & tests [Regular](option_regular.rs)
* Until now all of the examples relied on two key assumptions to serialize and deserialize a byte stream. These two assumptions are:
  1. Types have a well defined `size` in bytes required to represent them on the byte stream, less [type layout alignment](https://doc.rust-lang.org/reference/type-layout.html), and this size is know at compile time.
  1. Where the `size` is NOT known at compile time we were able to use `deplete` attribute to prevent `greedy` deserialization
  
* On the contrary `Option<T>` types have a `size` that can have two different states, `zero` or defined by one of two rules above. Hence to be able to deal with `Option<T>` 
on byte streams we introduce two new `byteserde` attributes, namely:

    * `#[byteserde(peek( start, len ))]` - this is a `struct` level attribute and us allows to peek into the byte stream and `yields` a byte slice `&[u8]`
    * `#[byteserde(eq( ... ))]` - this is a `field` level  attribute it allows us to define a byte slice expression which identifies specific member whose option type follows in the byte stream.


    * Example: Key considerations:
      * All optional elements must have a common byte or several bytes in a fixed location to differentiate it from other optional types, in this example it is one byte `u8` in first position
      * All Option<T> members must be defined in a single `struct` / section
      * `OptionalSection` member is `greedy` and must be annotated with `deplete` instruction to know when to stop deserialization
        ```rust
        #[derive(...)]
        struct Opt1{
            #[byteserde(replace( 1 ))]
            id: u8, 
            v: u16,
        }

        #[derive(...)]
        struct Opt2{
            #[byteserde(replace( 2 ))]
            id: u8, 
            v: u32, 
            v1: u64
        } // note that Opt2 only need to match on the id field and not the rest

        #[derive(...)]
        #[byteserde(peek( 0, 1 ))] // peek one byte, yields a slice `&[u8]` of len 1
        struct OptionalSection{
            // all members in this struct must be of Option<X>  form
            #[byteserde(eq( &[1] ))] // if peeked value eq to this expression deserialize as Opt1
            opt1: Option<Opt1>,

            #[byteserde(eq( &[2] ))] // if peeked value eq to this expression deserialize as Opt2
            opt2: Option<Opt2>,
        }

        #[derive(...)]
        struct WithOption{
            //... snip a number of none optional members

            #[byteserde(replace( optional_section.byte_len() ))]
            optional_section_length: u16,

            #[byteserde(deplete( optional_section_length ))]
            optional_section: OptionalSection,

            //... snip more none optional members
        }
        ```

## `Enum` support
* Comprehensive Examples & tests [Tuple](enum_like_tuple.rs) 
    * Please refer to the example provided for an overview but not that just like an optional section some part of the byte stream need to be able to identify which specific variant of the enum the stream should be deserialized into.

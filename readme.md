[![Push to master](https://github.com/softstream-link/byteserde/actions/workflows/push-master.yml/badge.svg)](https://github.com/softstream-link/byteserde/actions/workflows/push-master.yml)

# Motivation

* The motivation for this product is two fold:
  
  * To be able to map a `byte slice &[u8]` , typically acquired from a network, into a rust  `struct` datamodel by simply using  `derive macro` annotations and attributes auto-generate necessary code.
  
  * This comes very handy when the `byte slice` is not serialized using one of the existing and widely available protocols. Ex: An application specific [C-Struct](https://en.wikipedia.org/wiki/Struct_(C_programming_language)).
  
  * To be the `fastest` byte stream serializer/deserializer on the market for latency sensitive use cases. Benchmark results below show a performance summary of serializing & deserializing a [Reference Struct](byteserde_examples/benches/sample.rs#Numbers) using different frameworks available:
      * `byteserde` - `~15ns` read/write 
      * `bincode` - `~15ns` read / `~100ns` write
      * `rmp-serde` - `~215ns` read/write
      * `serde_json` - `~600ns` read/write - understandably slow due to strings usage
        * [this document](./byteserde_examples/readme.md) contains benchmark details.


# When to `use` and  when to `avoid` this framework
## Use
  
  * If you work with network protocols that deliver data in a `byte stream` format that is not matching one of the widely available standards, ex: `bincode`, `protobuf`. Use this product to efficiently map your `byte stream` into a rust  `struct`. 
  
  * You have a latency sensitive usecase. Note that this protocol does not add any schema information during serialization process and hence is equivalent of dumping the memory layout of the struct without padding
  
  * Example of protocols that are a perfect fit for this framework.
    * [Ouch5](http://nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/Ouch5.0.pdf)
    * [SoupBinTCP](https://www.nasdaq.com/docs/SoupBinTCP%204.1.pdf)
    * [Boe US Equities](https://cdn.cboe.com/resources/membership/Cboe_US_Equities_BOE_Specification.pdf)
    * .. etc

## Avoid
  * If the `byte stream` is serialized or deserialized using a wideley available standard avoid this framework and instead the that respective standard to work with the `byte stream`

# The project contains three craits
## [byteserde_derive@crates.io](https://crates.io/crates/byteserde_derive) - [byteserde_derive/Cargo.toml](byteserde_derive/Cargo.toml)
  * contains derive macros that generates [byteserde@crates.io](https://crates.io/crates/byteserde) traits
    * `#[derive(ByteSerializeStack)]` - generates [ByteSerializeStack trait](byteserde/src/ser_stack.rs#ByteSerializeStack) 
    
    * `#[derive(ByteSerializeHeap)]` - generates [ByteSerializeHeap trait](byteserde/src/ser_stack.rs#ByteSerializeHeap)
    
    * `#[derive(ByteDeserializeSlice)]` - generates [ByteDeserializeSlice`<T>` trait](byteserde/src/des_slice.rs#ByteDeserializeSlice)

    * `#[derive(ByteSerializedSizeOf)]` - generates [ByteSerializedSizeOf trait](byteserde/src/size.rs#ByteSerializedSizeOf) - this trait provides an `associated` method `byte_size()` which gives you a `struct` memory size in bytes without alignment. However it does not support types which heap allocate, ex: Vectors, Strings, or their derivations.
    
    * `#[derive(ByteSerializedLenOf)]` - generates  [ByteSerializedLenOf trait](byteserde/src/size.rs#ByteSerializedLenOf) - this trait provides an `instance` method `byte_len(&self)` which gives you memory size in bytes without alignment of specific instance. It exists specifically to deal with types that `ByteSerializedSizeOf trait` does not support
  * For more examples follow [here](byteserde_examples/examples/readme.md)
  * NOTE: that Union and Unit structure are not supported, but it might change in the future.
  
## [byteserde@crates.io](https://crates.io/crates/byteserde) - [byteserde/Cargo.toml](byteserde/Cargo.toml)            
* Highlights
  * [ByteSerialize***r***Stack`<CAP>`](byteserde/src/ser_stack.rs#ByteSerializerStack) - provides ultra fast serializer into a pre allocated `byte array` `[u8; CAP]` on `stack`, hence the name, it is very fast but at the cost of you needing to specify the size of the LARGEST `struct` you will attempt to serialize. If you reach the boundary of this preallocated byte array, your serialization will fail. This utility provides a reset features, which moves the internal counter to the begining, and allows you to recycle the buffer multiple times. 
    * works for `struct`s that implement [ByteSerializeStack trait](byteserde/src/ser_stack.rs#ByteSerializeStack)

  * [ByteSerialize***r***Heap](byteserde/src/ser_stack.rs#ByteSerializerHeap) - provides a fast enough for most speed by serializing into a `byte vector` `Vec<u8>`, hence the name. This utility trades some performance in return for not having to worry about knowing the LARGEST `struct` size in advance.
    * works for `struct`s that implement [ByteSerializeHeap trait](byteserde/src/ser_stack.rs#ByteSerializeHeap)

  * [ByteDeserialize***r***Slice](byteserde/src/des_slice.rs#ByteDeserializeSlice) - takes a `byte stream` `&[u8]` irrespctive of heap vs stack allocation and turns it into a `struct`
    * works for `struct`s that implement [ByteDeserializeSlice`<T>` trait](byteserde/src/des_slice.rs#ByteDeserializeslice)


    
## [byteserde_types@crates.io](https://crates.io/crates/byteserde_types) - [byteserde_types/Cargo.toml](byteserde_types/Cargo.toml)
  * contains optional ascii string related types and macros, which are typically usefull when dealing with fixed length strings while parsing a `byte stream`, follow [example section](byteserde_examples/examples/readme.md) for more details.

# Examples & Overview
* Please refer to [this document](byteserde_examples/examples/readme.md) for a number of comprehensive examples and features overview.



# Motivation

* The motivation for this product is two fold:
  * To be the `fastest` byte stream serializer/deserializer on the market for latency sensetive usecases.
  
  * To be able to define rust `struct` that represent application data models while at the same time not having to write code and instead using `derive` annotations and attributes auto-generate code to put these models on the wire for `new` or `existing` `latency` sensetive network protocols. Hence any and all auto generated serialization code should be as fast as one can possibly write by hand. 
  
* Benchmark results show a reference structure takes `~30ns` to read or write using this product, vs `~215ns` using `rmp-serde`, vs `~600ns` using `serde_json`. 
  * refer to [this document](./byteserde_examples/readme.md) for further details.


# Benefit case
* If you work with network protocols that deliver data in `byte stream` format and a well defined sequence you can use this product to quickly and efficently map your `byte stream` into a `struct` of your choice at zero performance cost and focus on the business logic instead of parsing and mapping.
  * Example:
    * [Ouch5](http://nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/Ouch5.0.pdf)
    * [SoupBinTCP](https://www.nasdaq.com/docs/SoupBinTCP%204.1.pdf)
    * etc..



# The project contains three craits
## [byteserde_derive@crates.io](https://crates.io/crates/byteserde_derive) - [byteserde_derive/Cargo.toml](byteserde_derive/Cargo.toml)
  * contains derive macros that generates [byteserde@crates.io](https://crates.io/crates/byteserde) traits
    * `#[derive(ByteSerializeStack)]` - generates [ByteSerializeStack trait](byteserde/src/ser.rs#ByteSerializeStack) 
    
    * `#[derive(ByteSerializeHeap)]` - generates [ByteSerializeHeap trait](byteserde/src/ser.rs#ByteSerializeHeap)
    
    * `#[derive(ByteDeserialize)]` - generates [ByteDeserialize`<T>` trait](byteserde/src/des.rs#ByteDeserialize)

    * `#[derive(ByteSerializedSizeOf)]` - generates [ByteSerializedSizeOf trait](byteserde/src/size.rs#ByteSerializedSizeOf) - this trait provides an `associated` method `byte_size()` which gives you a `struct` memory size in bytes without alignment. However it does not support types which heap allocate, ex: Vectors, Strings, or their derivations.
    
    * `#[derive(ByteSerializedLenOf)]` - generates  [ByteSerializedLenOf trait](byteserde/src/size.rs#ByteSerializedLenOf) - this trait provides an `instance` method `byte_len(&self)` which gives you memory size in bytes without alignment of specific instance. It exists specifically to deal with types tha `ByteSerializedSizeOf trait` does not support
  * For more examples follow [here](byteserde_examples/examples/readme.md)
  * NOTE: that Union and Unit structure are not supported, but it might change in the future.
  
## [byteserde@crates.io](https://crates.io/crates/byteserde) - [byteserde/Cargo.toml](byteserde/Cargo.toml)            
* Highlights
  * [ByteSerialize***r***Stack`<CAP>`](byteserde/src/ser.rs#ByteSerializerStack) - provides ultra fast serializer into a pre allocated `byte array` `[u8; CAP]` on `stack`, hence the name, it is very fast but at the cost of you needing to specify the size of the LARGEST `struct` you will attempt to serialize. If you reach the boundary of this preallocated byte array, your serialization will fail. This utility provides a reset features, which moves the internal counter to the begining, and allows you to recycle the buffer multiple times. 
    * works for `struct`s that implement [ByteSerializeStack trait](byteserde/src/ser.rs#ByteSerializeStack)

  * [ByteSerialize***r***Heap](byteserde/src/ser.rs#ByteSerializerHeap) - provides a fast enough for most speed by serializing into a `byte vector` `Vec<u8>`, hence the name. This utility trades some performance in return for not having to worry about knowing the LARGEST `struct` size in advance.
    * works for `struct`s that implement [ByteSerializeHeap trait](byteserde/src/ser.rs#ByteSerializeHeap)

  * [ByteDeserialize***r***](byteserde/src/des.rs#ByteDeserialize) - takes a `byte stream` `&[u8]` irrespctive of heap vs stack allocation and turns it into a `struct`
    * works for `struct`s that implement [ByteDeserialize`<T>` trait](byteserde/src/des.rs#ByteDeserialize)


    
## [byteserde_types@crates.io](https://crates.io/crates/byteserde_types) - [byteserde_types/Cargo.toml](byteserde_types/Cargo.toml)
  * contains optional ascii string related types and macros, which are typically usefull when dealing with fixed length strings while parsing a `byte stream`, follow [example section](byteserde_examples/examples/readme.md) for more details.

# Examples & Overview
* Please refer to [this document](byteserde_examples/examples/readme.md) for a number of comprehensive examples and features overview.



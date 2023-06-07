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



# Project Structure
* The project contains three craits
    * [byteserde@crates.io](https://crates.io/crates/byteserde) - [byteserde/Cargo.toml](byteserde/Cargo.toml)
        * contains [ByteSerializeStack](byteserde/src/ser.rs#ByteSerializeStack), [ByteSerializeHeap](byteserde/src/ser.rs#ByteSerializeHeap) & [ByteDeserialize`<T>`](byteserde/src/des.rs#ByteDeserialize) `traits` and helper `struct`'s that make it easy to manually create custom `byte stream` serailizer and deserializer
            
        * [ByteSerialize***r***Stack`<CAP>`](byteserde/src/ser.rs#ByteSerializerStack) - provides ultra fast speed by serializing into a pre allocated `byte array` `[u8; CAP]` on `stack`, hence the name, it is very fast but at the cost of you needing to specify the size of the LARGEST `struct` you will attempt to serialize. If you reach the boundary of this preallocated byte array, your serialization will fail. This utility provides a reset features, which moves the internal counter to the begining, and allows you to recycle the buffer for multiple purpoces. 
        * [ByteSerialize***r***Heap](byteserde/src/ser.rs#ByteSerializerHeap) - provides a fast enough for most speed by serializing into a `byte vector` `Vec<u8>`, hence the name. This utility trades some performance in return for not having to worry about knowing the LARGEST `struct` size in advance. 

        * [ByteDeserialize***r***](byteserde/src/des.rs#ByteDeserialize) - takes a `byte stream` `&[u8]` irrespctive of heap vs stack allocation and turns it into a `struct`

    * [byteserde_derive@crates.io](https://crates.io/crates/byteserde_derive) - [byteserde_derive/Cargo.toml](byteserde_derive/Cargo.toml)
        * contains procedural macro that generates `byteserde` trait implementations on `regular`, `tuple`, `enum` rust structure. 
        * NOTE: that Union and Unit structure are not supported ,this might change in the future.
    
    * [byteserde_types@crates.io](https://crates.io/crates/byteserde_types) - [byteserde_types/Cargo.toml](byteserde_types/Cargo.toml)
        * contains optional ascii string related types and macros, which are typically usefull when dealing with fixed length strings while parsing a `byte stream`, see examples section for more details.

# Examples & Overview
* Please refer to [this document](byteserde_examples/examples/readme.md) for a number of helpfull examples and feature review.



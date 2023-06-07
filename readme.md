# Motivation
* This product is for [bit steam](https://en.wikipedia.org/wiki/Bitstream) what [serde](https://serde.rs) is for [json](https://www.json.org)

* The goal of this product is to provide a set of utilities that enable frictionless transitioning between a `byte stream`, ex: `&[u8]`, and an arbitrary `struct`. In other words, the project provides a set of `traits` and `impl`'s that can be used to manually `serialize` an arbitrary `struct` into a `byte stream` as well as to `deserialize` a given `byte stream` into it original `struct`. 

* In addition to be able to custom serialize an arbitrary `struct`, you can leverage an included `#[derive(..)]` `proc_macro` and a number usefull `macro attributes` to create automatically generated serialize and deserialize `trait` implementation that covers most of typical usecases.


# Benefit case
* If you work with network streams which deliver data in `byte stream` format and a well defined sequence you can use this product to quickly and efficently map your `byte stream` into a `struct` of your choice and focus on the business logic instead of parsing and mapping.

* if you have two or more systems which need to communicate with each other, either over a network socket or a shared memory, but at a very `low latency`/`cpu cost`, this product is a good choice for you.


# Structure
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



# Motivation
* This product is for [bit steam](https://en.wikipedia.org/wiki/Bitstream) what [serde](https://serde.rs) is for [json](https://www.json.org)

* The goal of this product is to provide a set of utilities that enable frictionless transitioning between a `byte stream`, ex: `&[u8]`, and an arbitrary `struct`. In other words, the project provides a set of `traits` and `impl`'s that can be used to manually `serialize` an arbitrary `struct` into a `byte stream` as well as to `deserialize` a given `byte stream` into it original `struct`. 

* In addition to be able to custom serialize an arbitrary `struct`, you can leverage an included `#[derive(..)]` `proc_macro` and a number usefull `macro attributes` to create automatically generated serialize and deserialize `trait` implementation that covers most of typical usecases.


# Benefit case
* If you work with network streams which deliver data in `byte stream` format and a well defined sequence you can use this product to quickly and efficently map your `byte stream` into a `struct` of your choice and focus on the business logic instead of parsing and mapping.

* if you have two or more systems which need to communicate with each other, either over a network socket or a shared memory, but at a very `low latency`/`cpu cost`, this product is a good choice for you.


# Structure
* The project contains two cargo artifacts
    * `byteserde` - [Cargo.toml](Cargo.toml)
        * contains [ByteSerializeStack](src/ser.rs#ByteSerializeStack), [ByteSerializeHeap](src/ser.rs#ByteSerializeHeap) & [ByteDeserialize`<T>`](src/des.rs#ByteDeserialize) `traits` and helper `struct`'s that make it easy to manually create custom `byte stream` serailizer and deserializer
            
        * [ByteSerialize***r***Stack`<CAP>`](src/ser.rs#ByteSerializerStack) - provides ultra fast speed by serializing into a pre allocated `byte array` `[u8; CAP]` on `stack`, hence the name, it is very fast but at the cost of you needing to specify the size of the LARGEST `struct` you will attempt to serialize. If you reach the boundary of this preallocated byte array, your serialization will fail. This utility provides a reset features, which moves the internal counter to the begining, and allows you to recycle the buffer for multiple purpoces. 
        * [ByteSerialize***r***Heap](src/ser.rs#ByteSerializerHeap) - provides a fast enough for most speed by serializing into a `byte vector` `Vec<u8>`, hence the name. This utility trades some performance in return for not having to worry about knowing the LARGEST `struct` size in advance. 

        * [ByteDeserialize***r***](src/des.rs#ByteDeserialize) - takes a `byte stream` `&[u8]` irrespctive of heap vs stack allocation and turns it into a `struct`

    * `byteserde_derive` - [byteserde/Cargo.toml](byteserde/Cargo.toml)
        * contains procedural macro that generaters implementation of these traits on regular & tuple rust structure. 
        * NOTE: that Union, Enum, and Unit structure are not not currently supported

# Examples & Overview
* Please reffer to [this document](./examples/readme.md) for a number of helpfull examples and feature review.

# Want to contribute?
* Here are a few useful hints to get you started [developers corner](dev.md)


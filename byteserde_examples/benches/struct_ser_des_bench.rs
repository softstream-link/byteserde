use byteserde::prelude::*;
use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default)]
// #[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, Default)]
#[byteserde(endian = "le")]
pub struct StructHeaderInteger {
    type_i8: i8,
    #[byteserde(endian = "be")]
    type_u8: u8,
    type_i16: i16,
    #[byteserde(endian = "be")]
    type_u16: u16,
    type_i32: i32,
    #[byteserde(endian = "be")]
    type_u32: u32,
    type_i64: i64,
    #[byteserde(endian = "be")]
    type_u64: u64,
    type_i128: i128,
    #[byteserde(endian = "be")]
    type_u128: u128,
}

#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default)]
#[byteserde(endian = "le")]
pub struct StructFooterFloat {
    type_f32: f32,
    #[byteserde(endian = "be")]
    type_f64: f64,
}
#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default)]
pub struct StructBodyNested {
    type_header: StructHeaderInteger,
    type_footer: StructFooterFloat,
}

fn reset_byte_serialize_stack(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    let ser = &mut ByteSerializerStack::<128>::default();
    c.bench_function("byte_serialize_stack - reset ByteSerializerStack", |b| {
        b.iter(|| {
            black_box({
                ser.reset();
                let _ = inp.byte_serialize_stack(ser);
            })
        })
    });
}

fn new_byte_serialize_stack(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    c.bench_function("byte_serialize_stack - new ByteSerializerStack", |b| {
        b.iter(|| {
            black_box({
                let _: [u8; 128] = to_bytes_stack(&inp).unwrap();
            })
        })
    });
}

fn reset_byte_serialize_heap(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    let ser = &mut ByteSerializerHeap::default();
    c.bench_function("byte_serialize_heap - reset ByteSerializerHeap", |b| {
        b.iter(|| {
            black_box({
                ser.reset();
                let _ = inp.byte_serialize_heap(ser);
            })
        })
    });
}

fn new_byte_serialize_heap(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    c.bench_function("byte_serialize_heap - new ByteSerializerHeap", |b| {
        b.iter(|| {
            black_box({
                let _: Vec<u8> = to_bytes_heap(&inp).unwrap();
            })
        })
    });
}

fn reset_from_bytes(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    let ser: ByteSerializerStack<128> = to_serializer_stack(&inp).unwrap();
    let des = &mut ByteDeserializer::new(ser.as_slice());
    c.bench_function("from_bytes - reset ByteDeserializer", |b| {
        b.iter(|| {
            black_box({
                des.reset();
                let _ = StructBodyNested::byte_deserialize(des).unwrap();
                // let _ = from_bytes::<StructBodyNested>(&ser.as_slice());
            })
        })
    });
}

fn new_from_bytes(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    let ser: ByteSerializerStack<128> = to_serializer_stack(&inp).unwrap();
    c.bench_function("from_bytes - new ByteDeserializer", |b| {
        b.iter(|| {
            black_box({
                let _ = from_bytes::<StructBodyNested>(&ser.as_slice());
            })
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(1));
    targets =
    reset_from_bytes,
    reset_byte_serialize_stack,
    reset_byte_serialize_heap,
    new_from_bytes,
    new_byte_serialize_stack,
    new_byte_serialize_heap,
);
criterion_main!(benches);

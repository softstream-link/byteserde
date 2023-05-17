use byteserde::prelude::*;
// use byteserde_derive::{ByteDeserialize, ByteSerializeHeap, ByteSerializeStack};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default)]
pub struct StructHeaderInteger {
    type_i8: i8,
    type_u8: u8,
    type_i16: i16,
    type_u16: u16,
    type_i32: i32,
    type_u32: u32,
    type_i64: i64,
    type_u64: u64,
    type_i128: i128,
    type_u128: u128,
}

#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default)]
pub struct StructFooterFloat {
    type_f32: f32,
    type_f64: f64,
}
#[derive(Debug, PartialEq, ByteSerializeStack, ByteSerializeHeap, ByteDeserialize, Default)]
pub struct StructBodyNested {
    type_header: StructHeaderInteger,
    type_footer: StructFooterFloat,
}

fn bench_reuse_byte_serialize_stack(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    let ser = &mut ByteSerializerStack::<128>::default();
    c.bench_function("byte_serialize_stack - reuse ByteSerializerStack", |b| {
        b.iter(|| {
            black_box({
                ser.reset();
                let _ = inp.byte_serialize_stack(ser);
            })
        })
    });
}
fn bench_to_bytes_stack(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    c.bench_function("byte_serialize_stack - new ByteSerializerStack", |b| {
        b.iter(|| {
            black_box({
                let _: [u8; 128] = to_bytes_stack(&inp).unwrap();
            })
        })
    });
}

fn bench_reuse_byte_serialize_heap(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    let ser = &mut ByteSerializerHeap::default();
    c.bench_function("byte_serialize_heap - reuse ByteSerializerHeap", |b| {
        b.iter(|| {
            black_box({
                ser.reset();
                let _ = inp.byte_serialize_heap(ser);
            })
        })
    });
}

fn bench_to_bytes_heap(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    c.bench_function("byte_serialize_heap - new ByteSerializerHeap", |b| {
        b.iter(|| {
            black_box({
                let _: Vec<u8> = to_bytes_heap(&inp).unwrap();
            })
        })
    });
}

fn bench_from_bytes(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    let ser: ByteSerializerStack<128> = to_serializer_stack(&inp).unwrap();
    c.bench_function("from_bytes", |b| {
        b.iter(|| {
            black_box({
                let _ = from_bytes::<StructBodyNested>(&ser.bytes());
            })
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(1));
    targets =
    bench_reuse_byte_serialize_stack,
    bench_to_bytes_stack,
    bench_reuse_byte_serialize_heap,
    bench_to_bytes_heap,
    bench_from_bytes,
);
criterion_main!(benches);

mod sample;
use bytes::Bytes;
use byteserde::prelude::*;
use sample::Numbers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn reset_byte_serialize_stack(c: &mut Criterion) {
    let inp = Numbers::default();
    let ser = &mut ByteSerializerStack::<128>::default();
    c.bench_function("byte_serialize_stack - reset ByteSerializerStack", |b| {
        b.iter(|| {
            black_box({
                ser.clear();
                let _ = inp.byte_serialize_stack(ser).unwrap();
            })
        })
    });
}

fn new_byte_serialize_stack(c: &mut Criterion) {
    let inp = Numbers::default();
    c.bench_function("byte_serialize_stack - new ByteSerializerStack", |b| {
        b.iter(|| {
            black_box({
                let _: ([u8; 128], usize) = to_bytes_stack(&inp).unwrap();
            })
        })
    });
}

fn reset_byte_serialize_heap(c: &mut Criterion) {
    let inp = Numbers::default();
    let ser = &mut ByteSerializerHeap::default();
    c.bench_function("byte_serialize_heap - reset ByteSerializerHeap", |b| {
        b.iter(|| {
            black_box({
                ser.clear();
                let _ = inp.byte_serialize_heap(ser).unwrap();
            })
        })
    });
}

fn new_byte_serialize_heap(c: &mut Criterion) {
    let inp = Numbers::default();
    c.bench_function("byte_serialize_heap - new ByteSerializerHeap", |b| {
        b.iter(|| {
            black_box({
                let _: Bytes = to_bytes_heap(&inp).unwrap();
            })
        })
    });
}

fn reset_from_bytes(c: &mut Criterion) {
    let inp = Numbers::default();
    let ser: ByteSerializerStack<128> = to_serializer_stack(&inp).unwrap();
    let des = &mut ByteDeserializerSlice::new(ser.as_slice());
    c.bench_function("from_bytes - reset ByteDeserializer", |b| {
        b.iter(|| {
            black_box({
                des.reset();
                let _ = <Numbers as ByteDeserializeSlice<Numbers>>::byte_deserialize(des).unwrap();
                // let _ = from_bytes::<Numbers>(&ser.as_slice());
            })
        })
    });
}

fn new_from_bytes(c: &mut Criterion) {
    let inp = Numbers::default();
    let ser: ByteSerializerStack<128> = to_serializer_stack(&inp).unwrap();
    c.bench_function("from_bytes - new ByteDeserializer", |b| {
        b.iter(|| {
            black_box({
                let _: Numbers = from_slice(&ser.as_slice()).unwrap();
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

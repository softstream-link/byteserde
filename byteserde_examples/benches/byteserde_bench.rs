mod sample;
use bytes::Bytes;
use byteserde::prelude::*;
use sample::Numbers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn to_bytes(c: &mut Criterion) {
    let inp = Numbers::default();
    c.bench_function("byteserde::to_bytes_stack", |b| {
        b.iter(|| {
            black_box({
                let _: ([u8; 128], usize) = byteserde::prelude::to_bytes_stack(&inp).unwrap();
            })
        })
    });
}

fn from_slice(c: &mut Criterion) {
    let inp = Numbers::default();
    let ser: ByteSerializerStack<128> = to_serializer_stack(&inp).unwrap();
    c.bench_function("byteserde::from_slice", |b| {
        b.iter(|| {
            black_box({
                let _ : Numbers = byteserde::prelude::from_slice(&ser.as_slice()).unwrap();
            })
        })
    });
}

fn from_bytes(c: &mut Criterion) {
    let inp = Numbers::default();
    let ser: ByteSerializerStack<128> = to_serializer_stack(&inp).unwrap();
    let bytes: Bytes = ser.as_slice().to_vec().into();
    c.bench_function("byteserde::from_bytes", |b| {
        b.iter( || {
            black_box({
                let _ : Numbers = byteserde::prelude::from_bytes(bytes.clone()).unwrap();
            })
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(5));
    targets =
    to_bytes,
    from_slice,
    from_bytes,
);
criterion_main!(benches);

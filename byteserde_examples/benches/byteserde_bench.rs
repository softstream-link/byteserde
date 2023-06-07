mod common;
use byteserde::prelude::*;
use common::StructBodyNested;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn to_bytes(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    c.bench_function("byteserde::to_bytes_stack", |b| {
        b.iter(|| {
            black_box({
                let _: [u8; 128] = byteserde::prelude::to_bytes_stack(&inp).unwrap();
            })
        })
    });
}

fn from_bytes(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    let ser: ByteSerializerStack<128> = to_serializer_stack(&inp).unwrap();
    c.bench_function("byteserde::from_bytes", |b| {
        b.iter(|| {
            black_box({
                let _ : StructBodyNested = byteserde::prelude::from_bytes(&ser.as_slice()).unwrap();
            })
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(1));
    targets =
    to_bytes,
    from_bytes,
);
criterion_main!(benches);

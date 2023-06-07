mod common;
use common::StructBodyNested;
// use rmp_serde::{Deserializer, Serializer};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn to_vec(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    c.bench_function("rmp_serde::to_vec", |b| {
        b.iter(|| {
            black_box({
                let _ = rmp_serde::to_vec(&inp).unwrap();
            })
        })
    });
}

fn from_vec(c: &mut Criterion) {
    let inp = StructBodyNested::default();
    let rmp = rmp_serde::to_vec(&inp).unwrap();
    c.bench_function("rmp_serde::from_read", |b| {
        b.iter(|| {
            black_box({
                let _ : StructBodyNested = rmp_serde::from_read(rmp.as_slice()).unwrap();
            })
        })
    });
}



 
criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(1));
    targets =
    to_vec,
    from_vec,
);
criterion_main!(benches);

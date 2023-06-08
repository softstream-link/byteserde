mod sample;
use sample::Numbers;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn to_vec(c: &mut Criterion) {
    let inp = Numbers::default();
    c.bench_function("bincode::serialize", |b| {
        b.iter(|| {
            black_box({
                let _ = bincode::serialize(&inp).unwrap();
            })
        })
    });
}

fn from_vec(c: &mut Criterion) {
    let inp = Numbers::default();
    let bincode = bincode::serialize(&inp).unwrap();
    c.bench_function("bincode::deserialize", |b| {
        b.iter(|| {
            black_box({
                let _: Numbers = bincode::deserialize(bincode.as_slice()).unwrap();
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

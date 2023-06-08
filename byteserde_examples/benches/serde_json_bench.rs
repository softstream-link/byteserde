mod sample;
use sample::Numbers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn to_string(c: &mut Criterion) {
    let inp = Numbers::default();
    c.bench_function("serde_json::to_string", |b| {
        b.iter(|| {
            black_box({
                let _ = serde_json::to_string(&inp).unwrap();
            })
        })
    });
}

fn from_string(c: &mut Criterion) {
    let inp = Numbers::default();
    let json = serde_json::to_string(&inp).unwrap();
    c.bench_function("serde_json::from_str", |b| {
        b.iter(|| {
            black_box({
                let _ : Numbers = serde_json::from_str(&json).unwrap();
            })
        })
    });
}



 
criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(1));
    targets =
    to_string,
    from_string,
);
criterion_main!(benches);

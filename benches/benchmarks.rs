use criterion::{criterion_group, criterion_main, Criterion};
use ethers::types::H160;

pub fn count_leading_zero_bytes_benchmark(c: &mut Criterion) {
    let zero_address = H160::zero();

    c.bench_function("Count leading zero bytes", |b| {
        b.iter(|| zero_seeker::count_leading_zero_bytes(&zero_address))
    });
}

pub fn count_zero_bytes_benchmark(c: &mut Criterion) {
    let zero_address = H160::zero();

    c.bench_function("Count zero bytes", |b| {
        b.iter(|| zero_seeker::count_zero_bytes(&zero_address))
    });
}

criterion_group!(
    benches,
    count_leading_zero_bytes_benchmark,
    count_zero_bytes_benchmark
);

criterion_main!(benches);

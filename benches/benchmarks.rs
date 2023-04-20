use criterion::{criterion_group, criterion_main, Criterion};

pub fn count_leading_zero_bytes_benchmark(c: &mut Criterion) {
    c.bench_function("Count leading zero bytes", |b| b.iter(|| todo!()));
}

pub fn count_zero_bytes_benchmark(c: &mut Criterion) {
    c.bench_function("Count zero bytes", |b| b.iter(|| todo!()));
}

criterion_group!(
    benches,
    count_leading_zero_bytes_benchmark,
    count_zero_bytes_benchmark
);

criterion_main!(benches);

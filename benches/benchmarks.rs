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

pub fn find_optimal_batch_size_benchmark(c: &mut Criterion) {
    const SEED: &str = "0101010101";
    c.bench_function("Find optimal batch size", |b| {
        b.iter(|| zero_seeker::find_optimal_batch_size(SEED, false))
    });
}

pub fn mine_address_with_n_zero_bytes_benchmark(c: &mut Criterion) {
    const SEED: &str = "0101010101";

    c.bench_function("Mine address with 2 zero bytes", |b| {
        b.iter(|| zero_seeker::mine_address_with_n_zero_bytes(SEED, 2, false, 200))
    });
}

// pub fn mine_address_with_n_leading_zero_bytes_benchmark(c: &mut Criterion) {
//     const SEED: &str = "0101010101";
//     c.bench_function("Mine address with 2 zero bytes", |b| {
//         b.iter(|| zero_seeker::mine_address_with_n_zero_bytes(SEED, 2, true))
//     });
// }

criterion_group!(
    benches,
    // count_leading_zero_bytes_benchmark,
    // count_zero_bytes_benchmark,
    // find_optimal_batch_size_benchmark,
    mine_address_with_n_zero_bytes_benchmark
);

criterion_main!(benches);

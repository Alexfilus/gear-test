use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/parallel.rs"]
mod parallel;

use parallel::parallel_computation;
fn benchmark(c: &mut Criterion) {
    let func_mul = |x: u64| x * x;

    c.bench_function("parallel_computation_1000", |b| {
        b.iter(|| {
            let result = parallel_computation(black_box((1..=1000).collect()), func_mul, 1000);
            black_box(result);
        })
    });

    c.bench_function("parallel_computation_5000", |b| {
        b.iter(|| {
            let result = parallel_computation(black_box((1..=5000).collect()), func_mul, 1000);
            black_box(result);
        })
    });

    c.bench_function("parallel_computation_50000", |b| {
        b.iter(|| {
            let result = parallel_computation(black_box((1..=50000).collect()), func_mul, 1000);
            black_box(result);
        })
    });

    c.bench_function("parallel_computation_500000", |b| {
        b.iter(|| {
            let result = parallel_computation(black_box((1..=500000).collect()), func_mul, 1000);
            black_box(result);
        })
    });

    c.bench_function("parallel_computation_500000_one_thread", |b| {
        b.iter(|| {
            let result = parallel_computation(black_box((1..=500000).collect()), func_mul, 500000);
            black_box(result);
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

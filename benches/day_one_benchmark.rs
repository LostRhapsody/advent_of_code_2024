use advent_of_code_2024::day_one::solve;
use criterion::{criterion_group, criterion_main, Criterion};

fn day_one_benchmark(c: &mut Criterion) {
    c.bench_function("day_one", |b| b.iter(|| solve()));
}

criterion_group!(benches, day_one_benchmark);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use aoc::year2023::day01::{solution1, solution2};

fn benchmark_solutions(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/year2023/day01/input.txt")
        .expect("Failed to read input");

    let mut group = c.benchmark_group("day01_solutions");

    // Benchmark Solution 1
    group.bench_function("Solution 1", |b| {
        b.iter(|| solution1::run(&input));
    });

    // Benchmark Solution 2
    group.bench_function("Solution 2", |b| {
        b.iter(|| solution2::run(&input));
    });

    group.finish();
}

criterion_group!(benches, benchmark_solutions);
criterion_main!(benches);


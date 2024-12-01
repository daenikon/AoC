use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use aoc::year2024::day01::solution;

fn benchmark_solutions(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/year2024/day01/input.txt")
        .expect("Failed to read input");

    let mut group = c.benchmark_group("year2024_day01_solutions");

    group.bench_function("Solution 1", |b| {
        b.iter(|| solution::run(&input));
    });

    group.finish();
}

criterion_group!(benches, benchmark_solutions);
criterion_main!(benches);


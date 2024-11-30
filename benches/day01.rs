use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use aoc::year2023::day01::solution;

fn benchmark_solution(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/year2023/day01/input.txt")
        .expect("Failed to read input");

    c.bench_function("day01_solution", |b| {
        b.iter(|| {
            let result = solution::run(black_box(&input)); // Use black_box to prevent optimization
            black_box(result); // Prevent compiler optimization of the result
        });
    });
}

criterion_group!(benches, benchmark_solution);
criterion_main!(benches);


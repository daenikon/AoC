use std::fs;
use aoc::year2023::day01::{solution1, solution2};

fn main() {
    let input = fs::read_to_string("./inputs/year2023/day01/input.txt")
        .expect("Failed to load input file");

    let result1 = solution1::run(&input);
    println!("Solution 1 Result: {}", result1);

    let result2 = solution2::run(&input);
    println!("Solution 2 Result: {}", result2);
}


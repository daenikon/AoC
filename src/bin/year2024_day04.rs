use std::fs;
use aoc::year2024::day04::solution;

fn main() {
    let input = fs::read_to_string("./inputs/year2024/day04/input.txt")
        .expect("Failed to load input file");

    let result = solution::run(&input);
    println!("Solution Result: {}", result);
}


use std::fs;
//use aoc::year2024::day03::solution;
use aoc::year2024::day03::solution2;

fn main() {
    let input = fs::read_to_string("./inputs/year2024/day03/input.txt")
        .expect("Failed to load input file");

    let result = solution2::run(&input);
    println!("Solution Result: {}", result);
}


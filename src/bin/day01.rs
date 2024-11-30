use std::fs;
use aoc::year2023::day01::solution;

fn main() {
    let input = fs::read_to_string("./inputs/year2023/day01/input.txt")
        .expect("Failed to load input file");

    println!("{}", solution::run(&input));
}


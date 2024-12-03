use std::fs;
//use aoc::year2024::day02::solution1;
use aoc::year2024::day02::solution2;

fn main() {
    let input = fs::read_to_string("./inputs/year2024/day02/input.txt")
        .expect("Failed to load input file");

    //let result = solution::run(&input);
    let result = solution2::run(&input);
    println!("Solution Result: {}", result);
}


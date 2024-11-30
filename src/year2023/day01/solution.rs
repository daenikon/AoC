//use std::fs;

pub fn run(contents: &str) -> u64 {
    //let contents = fs::read_to_string("./inputs/year2023/day01/input.txt").expect("Failed to load file's content");

    let char_nums = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut sum: u64 = 0;

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        let mut first_num:char = 'x'; // dummy value
        let mut last_num:char = 'x';
        for line_char in line.chars() {
            if char_nums.contains(&line_char) {
                last_num = line_char;

                if !char_nums.contains(&first_num) {
                    first_num = line_char;
                }
            }
        }
        sum += (first_num as u64 - 48) * 10;
        sum += last_num as u64 - 48;
    }
    sum
}

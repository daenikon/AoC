use regex::Regex;

pub fn run(input: &str) -> u16 {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    if let Some(caps) = pattern.captures(input) {
        println!("Full match: {}", &caps[0]);
        println!("First number: {}", &caps[1]);
        println!("Second number: {}", &caps[2]);
    } else {
        println!("No match found!");
    }
    0
}

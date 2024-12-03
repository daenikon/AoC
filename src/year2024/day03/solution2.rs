use regex::Regex;

pub fn run(hay: &str) -> u32 {
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();

    let mut is_enabled: bool = true;
    let mut product: u32 = 0;

    for caps in re.captures_iter(hay) {
        let first_group = caps.get(1).unwrap().as_str();
        if first_group == "do()" {
            is_enabled = true;
        } else if first_group == "don't()" {
            is_enabled = false;
        } else if is_enabled {
            let num1 = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let num2 = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
            product += num1 * num2;
        }
    }
    product
}

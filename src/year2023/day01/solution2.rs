// a bit more optimized
pub fn run(contents: &str) -> u32 {
    let char_nums = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut sum: u32 = 0;

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        let mut first_num:char = 'x';
        let mut last_num:char = 'x';
        for line_char in line.chars() {
            if char_nums.contains(&line_char) {
                first_num = line_char;
                break;
            }
        }

        for line_char in line.chars().rev() {
            if char_nums.contains(&line_char) {
                last_num = line_char;
                break;
            }
        }
        sum += (first_num as u32 - 48) * 10;
        sum += last_num as u32 - 48;
    }
    sum
}


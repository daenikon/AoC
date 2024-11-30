fn main() {
    let input = String::from("ab1231255469920cd1e5fgh");

    let char_nums = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut first_num:char = 'x'; // dummy value
    let mut last_num:char = 'x';

    for input_char in input.chars() {
        if char_nums.contains(&input_char) {
            last_num = input_char;

            if !char_nums.contains(&first_num) {
                first_num = input_char;
            }
        }
    }

    println!("{first_num}{last_num}");
}

pub fn run(input: &str) -> i32 {

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let num1 = &line[0..5].parse::<i32>().unwrap();
        let num2 = &line[8..13].parse::<i32>().unwrap();
        vec1.push(*num1);
        vec2.push(*num2);
        //println!("{} and {}", vec1[0], vec2[0]);
    }

    println!("{}", vec1.len());
    vec1.sort();
    vec2.sort();


    let mut sum_of_diffs: i32 = 0;

    for i in 0..1000 {
        let diff = (vec1[i] - vec2[i]).abs();
        //println!("Num1: {} | Num2: {} | Diff: {} | i: {}", vec1[i], vec2[i], diff, i);
        sum_of_diffs += diff;

        //println!("Num1: {}, Num2: {}, diff: {}", vec1[i], vec2[i], diff);
    }
    return sum_of_diffs as i32;
}

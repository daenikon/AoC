use regex::Regex;

pub fn run(hay: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    //let mat = re.find(hay).unwrap();
    //println!("Result: {}", mat.as_str());
    //let caps = re.captures(hay).unwrap();
    //println!("{}", caps.get(2).unwrap().as_str());

    let mut sum: u32 = 0;
    for (_, [mat1, mat2]) in re.captures_iter(hay).map(|c| c.extract()) {
        let num1: u32 = mat1.parse::<u32>().unwrap();
        let num2: u32 = mat2.parse::<u32>().unwrap();
        println!("Num1: {}, Num2: {}", num1, num2);
        sum += num1 * num2;
    }
    sum
}

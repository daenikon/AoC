use std::collections::HashMap;

pub fn run(input: &str) -> u32 {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let num1 = &line[0..5].parse::<u32>().unwrap();
        let num2 = &line[8..13].parse::<u32>().unwrap();
        vec1.push(*num1);
        vec2.push(*num2);
        //println!("{} and {}", vec1[0], vec2[0]);
    }

    //println!("{}", vec1.len());
    vec1.sort();
    vec2.sort();

    let total_distance: u32 = calculate_total_distance(&vec1, &vec2);
    println!("Total distance: {}", total_distance);

    let similarity_score: u32 = calculate_similarity_score(&vec1, &vec2);
    println!("Similarity score: {}", similarity_score);

    return total_distance as u32;
}

// Input: sorted vectors
fn calculate_total_distance(vec1: &Vec<u32>, vec2: &Vec<u32>) -> u32 {
    let mut total_distance: u32 = 0;

    for i in 0..vec1.len() {
        let distance = vec1[i].abs_diff(vec2[i]);
        total_distance += distance;
    }

    total_distance
}

fn calculate_similarity_score(vec1: &Vec<u32>, vec2: &Vec<u32>) -> u32 {
    let mut occurences = HashMap::new();
    for i in 0..vec2.len() {
        let temp = occurences.get(&vec2[i]).copied().unwrap_or(0);
        occurences.insert(vec2[i], temp + 1);
    }

    let mut similarity_score: u32 = 0;
    for i in 0..vec1.len() {
        if (i < vec1.len() - 2 && vec1[i] == vec1[i+1]) {
            continue;
        }
        let temp = occurences.get(&vec1[i]).copied().unwrap_or(0);
        similarity_score += temp * vec1[i];
    }
    similarity_score
}

pub fn run(input: &str) -> u16 {
    let mut safe_record_count: u16 = 0;

    for report in input.lines() {
        if report.is_empty() {
            continue;
        }

        if is_report_safe(&report) == true {
            safe_record_count += 1;
            println!("Report {} is safe", report);
        } else {
            println!("Report {} isn't safe", report);
        }
    }
    safe_record_count
}

// report is a string slice
fn is_report_safe(report: &str) -> bool {
    let levels: Vec<u8> = report
        .split_whitespace()
        .map(|s| s.parse::<u8>().unwrap()) // parse each string to an integer
        .collect();

    // validate first-second level difference
    let first_level_pair_diff: u8 = levels[0].abs_diff(levels[1]);
    if first_level_pair_diff == 0 || first_level_pair_diff > 3 {
        return false;
    }

    // set level order - ascending or descending
    let is_order_ascending: bool = levels[0] < levels[1];

    // start with the second level, since first diff was validated
    for i in 1..levels.len()-1 {
        let diff: u8 = levels[i].abs_diff(levels[i+1]);
        // validate level-pair
        if (levels[i] < levels[i+1]) != is_order_ascending || diff == 0 || diff > 3 {
            return false;
        }
    }
    true
}

pub fn run(input: &str) -> u16 {
    let mut safe_record_count: u16 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let report: Vec<u8> = line
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap()) // parse each string to an integer
            .collect();

        
        let report_data = validate_report(&report);
        if report_data.0 == true {
            safe_record_count += 1;
            //println!("Report {:?} is safe", report);
        } else {
            // More or less optimized
            for i in report_data.1-1..report_data.1+1 {
                let mut new_report = report.clone();
                new_report.remove(i.try_into().unwrap());
                if validate_report(&new_report).0 {
                    safe_record_count += 1;
                    break;
                }

                if report_data.1 == 2 {
                    let mut edge_case_report = report.clone();
                    edge_case_report.remove(0);
                    if validate_report(&edge_case_report).0 {
                        safe_record_count += 1;
                        break;
                    }
                }
            }
            /*//Brute force
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if validate_report(&new_report).0 {
                    safe_record_count += 1;
                    is_report_safe = true;
                    break;
                }
            }
            */
            
        }
    }
    safe_record_count
}

// report is a string slice
fn validate_report(report: &Vec<u8>) -> (bool, i16) {
    // validate first-second level difference
    let first_level_pair_diff: u8 = report[0].abs_diff(report[1]);
    if first_level_pair_diff == 0 || first_level_pair_diff > 3 {
        return (false, 1);
    }

    // set level order - ascending or descending
    let is_order_ascending: bool = report[0] < report[1];

    // start with the second level, since first diff was validated
    for i in 1..report.len()-1 {
        let diff: u8 = report[i].abs_diff(report[i+1]);
        // validate level-pair
        if (report[i] < report[i+1]) != is_order_ascending || diff == 0 || diff > 3 {
            return (false, (i+1) as i16);
        }
    }
    (true, -1)
}

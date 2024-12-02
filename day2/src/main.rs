use std::fs::File;
use std::io::{self, BufRead};

fn read_input_file(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let number_array: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();
        reports.push(number_array);
    }
    Ok(reports)
}

fn safe_checker(array: &Vec<i32>) -> bool {
    let mut ascending = array[0] < array[1];

    if array.len() < 2 {
        // Check length not less than 2
        return false;
    }

    for (current, next) in array.windows(2).map(|n| (n[0], n[1])) {
        let diff = (current - next).abs();
        // Check if the difference is not between 1 and 3
        if diff < 1 || diff > 3 {
            return false;
        }

        // Check trend direction
        if (current < next && !ascending) || (current > next && ascending) || (current == next) {
            return false;
        }
        ascending = current < next;
    }
    true
}

fn part1() {
    let file_path = "day2_input.txt";
    let report = match read_input_file(file_path) {
        Ok(reports) => reports,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut part_1_safe_reports = 0;

    for array in report.iter() {
        if safe_checker(&array) {
            part_1_safe_reports += 1;
        }
    }
    println!("Part 1: {}", part_1_safe_reports);
}

fn part2() {
    let mut part_2_safe_reports = 0;
    let file_path = "day2_input.txt";
    let report = match read_input_file(file_path) {
        Ok(reports) => reports,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    for array in report.iter() {
        // If the report is already safe in Part 1, count it as part of Part 2
        if safe_checker(&array) {
            part_2_safe_reports += 1;
        } else {
            // Part 2: Try removing one element at a time
            for i in 0..array.len() {
                let mut report_less_one = array.clone();
                report_less_one.remove(i);

                if safe_checker(&report_less_one) {
                    part_2_safe_reports += 1;
                    break; // Stop checking further once we find a valid configuration
                }
            }
        }
    }
    println!("Part 2: {}", part_2_safe_reports);
}

fn main() {
    part1();
    part2();
}
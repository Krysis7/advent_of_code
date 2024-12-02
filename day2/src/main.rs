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

fn part1() {
    let file_path = "day2_input.txt";

    let report = match read_input_file(file_path) {
        Ok(reports) => reports,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut success = 0;

    for array in report.iter() {
        if array.len() < 2 {
            // check length not less than 2;
            break;
        }

        // check trend for first 2 value
        let mut ascending = array[0] < array[1];
        let mut valid = true;

        for (current, next) in array.windows(2).map(|n| (n[0], n[1])) {
            let diff = (current - next).abs();
            // check greater then 3
            if  diff < 1 || diff > 3 {
                valid = false;
                // fails the check
                break;
            }

            // check trend direction
            if (current < next && !ascending) || (current > next && ascending) || (current == next) {
                valid = false;
                break;
            }
            ascending = current < next;
        }
        if valid {
            success += 1;
        }
    }
    println!("{}", success);
}

fn main() {
    part1();
}

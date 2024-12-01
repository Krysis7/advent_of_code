use std::fs::File;
use std::io::{self, BufRead};

fn read_input_file(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut array1 = Vec::new();
    let mut array2 = Vec::new();

    for line in reader.lines() {
        let line = line?; // error if it fails
        let mut split = line.split_whitespace();

        array1.push(split.next().unwrap().parse().unwrap());
        array2.push(split.next().unwrap().parse().unwrap());

    }

    Ok((array1, array2))
}

fn main() {
    let file_path = "day1_input.txt";

    let (mut array1, mut array2) = match read_input_file(file_path) {
        Ok((arr1, arr2)) => (arr1, arr2),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // sort the arrays
    array1.sort();
    array2.sort();

    // zip the value as tuple map and calculate difference using absolute value
    let distance: i32 = array1.iter()
        .zip(array2.iter())
        .map(|(i, y) | i32::abs(*i - *y))
        .sum();

    println!("{}", distance);
}

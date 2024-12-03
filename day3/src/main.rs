use regex::Regex;
use std::fs::File;
use std::io::{Read};

fn part1() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let file_path = "day3_input.txt";
    let mut file = File::open(file_path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    let products: Vec<i32> = re
        .captures_iter(&content)
        .map(|caps| {
            let x: i32 = caps[1].parse().unwrap(); 
            let y: i32 = caps[2].parse().unwrap();
            x * y
        })
        .collect();
    let total: i32 = products.iter().sum();
    println!("{}", total);
}

fn multiply_from_string(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = re.captures(input).unwrap(); // Unwrap because we know it will match
    let x: i32 = caps[1].parse().unwrap();
    let y: i32 = caps[2].parse().unwrap();
    x * y
}

fn part2() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't").unwrap();

    let file_path = "day3_input.txt";
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let products: Vec<String> = re
        .find_iter(&content)
        .map(|mat| mat.as_str().to_string())
        .collect();

    let mut multiply = true;
    let mut total = 0;

    for item in products {
        if item == "don't" {
            multiply = false; // Disable multiplication
        } else if item == "do()" {
            multiply = true; // Enable multiplication
        }

        if multiply && item.starts_with("mul") {
            total += multiply_from_string(&item);
        }
    }
    println!("{}", total);
}
fn main() {
    part1();
    part2();
}

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

fn main() {
    part1();
}

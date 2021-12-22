use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let nums = read_lines("input.txt");

    // Counting # of times most recent value greater than 4th most recent
    let mut total = 0;
    for i in 0..nums.len()-3 {
        if nums[i+3] > nums[i] { total += 1 };
    }

    println!("{}", total);
}

fn read_lines<P>(filename: P) -> Vec<i32> where P: AsRef<Path> {
    // Converts input lines to vector of integers
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    return lines
    .map(|l| l.unwrap().parse::<i32>().unwrap())
    .collect::<Vec<_>>();
}
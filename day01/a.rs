use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let nums = read_lines("input.txt");

    // Count # of times current value greater than last
    let mut prev = nums[0];
    let mut total = 0;
    for num in nums[1..].iter() {  // Already seen first num
        if num > &prev { total += 1; }
        prev = *num;
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
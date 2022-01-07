use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let bools_vec = read_lines("input.txt");

    // Determine most common bit in each index to form bit vector
    let mut max_vec = vec![];
    for i in 0..bools_vec[0].len() {
        
        let mut freq = 0;
        for j in 0..bools_vec.len() {
            if bools_vec[j][i] { freq += 1; }
        }
        max_vec.push(freq > bools_vec.len() / 2);
    }

    // Calculate both values from the bit vector
    let mut val = 0;
    let mut neg_val = 0;
    for b in max_vec {
        if b {
            val = val * 2 + 1; 
            neg_val *= 2;
        } else {
            val *= 2; 
            neg_val = neg_val * 2 + 1;
        }
    }

    println!("{}", val * neg_val);
}

fn read_lines<P>(filename: P) -> Vec<Vec<bool>> where P: AsRef<Path> {
    // Converts input lines to 2D vector of bool values
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    // Cells replaced with true for 1 and false for 0
    return lines.map(
        |s| s.unwrap().chars().map(
            |c| c == '1'
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
}
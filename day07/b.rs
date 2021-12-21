use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::cmp;

fn main() -> () {
    let positions = read_lines("input.txt");

    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    let mut min_tot = -1;

    for align in *min..=*max {
        // Normalise around the alignment point and sum displacement
        // This time use a triangular number metric
        let tot = positions.iter()
        .cloned()
        .map(|x| x - align)
        .fold(0, |acc, x| acc + (x.abs() * (x.abs() + 1)) / 2);

        // Always assign total on first pass, otherwise take the minimum
        if align == *min { min_tot = tot; }
        else { min_tot = cmp::min(min_tot, tot); }
    }
    println!("{}", min_tot);
}

fn read_lines<P>(filename: P) -> Vec<i32> where P: AsRef<Path> {
    // Converts input line to a vector of integers
    
    let file = File::open(filename).unwrap();
    let mut line = String::new();
    io::BufReader::new(file).read_line(&mut line).unwrap();

    return line
    .split(",")
    .map(|x| x.trim().parse::<i32>().unwrap())
    .collect::<Vec<_>>();
}
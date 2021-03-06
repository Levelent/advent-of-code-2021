use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let initial = read_lines("input.txt");

    // Convert initial fish states into bins of size 9
    let mut timers = vec![];
    for i in 0..9 {
        timers.push(initial.iter().filter(|x| **x == i).count());
    }

    // Apply reproduction steps 80 times, then continue to 256 times
    for j in 0..256 {
        if j == 80 {
            let num1 = timers.iter().cloned().fold(0, |acc, x| acc + x);
            println!("Part 1: {}", num1);
        }
        let replicate = timers[0];
        for i in 0..8 { timers[i] = timers[i + 1]; }
        timers[6] += replicate;
        timers[8] = replicate;
    }

    // Get the number of fish
    let num2 = timers.iter().fold(0, |acc, x| acc + x);
    println!("Part 2: {}", num2);
}

fn read_lines<P>(filename: P) -> Vec<usize> where P: AsRef<Path> {
    // Converts input line to a vector of integers
    
    let file = File::open(filename).unwrap();
    let mut line = String::new();
    io::BufReader::new(file).read_line(&mut line).unwrap();

    return line
    .split(",")
    .map(|x| x.trim().parse::<usize>().unwrap())
    .collect::<Vec<_>>();
}
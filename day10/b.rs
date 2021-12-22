use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

fn main() -> () {
    let char_grid = read_lines("input.txt");

    // Matching bracket pairs
    let brackets = HashMap::from([
        ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')
    ]);

    // Incomplete bracket scores
    let br_val = HashMap::from([
        (')', 1), (']', 2), ('}', 3), ('>', 4)
    ]);
    
    let mut totals: Vec<usize> = vec![];
    for row in char_grid {
        // Determine validity of chunk
        let mut stack = vec![];
        let mut valid = true;
        for i in 0..row.len() {
            // Check whether we need to push or pop
            match brackets.keys().find(|x| **x == row[i]) {
                Some(_) => { stack.push(row[i]); }
                None    => {
                    if stack.len() == 0 { panic!(); }
                    let rb = stack.pop().unwrap();
                    
                    // Check if invalid bracket used
                    if brackets[&rb] != row[i] { 
                        valid = false; 
                        break; 
                    }
                }
            }
        }

        if valid {
            // Calculate score required for completion
            stack.reverse();
            totals.push(
                stack.iter()
                .map(|x| br_val[&brackets[x]])
                .fold(0, |acc, x| (acc * 5) + x)
            );
        }
    }

    // Find median value of scores
    totals.sort();
    println!("{}", totals[totals.len() / 2]);
}

fn read_lines<P>(filename: P) -> Vec<Vec<char>> where P: AsRef<Path> {
    // Converts input lines to 2D vector of char values
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    return lines
    .map(|l| l.unwrap().chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
}
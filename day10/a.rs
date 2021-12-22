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

    // Incorrect bracket scores
    let br_val = HashMap::from([
        (')', 3), (']', 57), ('}', 1197), ('>', 25137)
    ]);
    
    let mut tot = 0;
    for row in char_grid {
        // Add score if an invalid chunk
        let mut stack = vec![];
        for i in 0..row.len() {
            // Check whether we need to push or pop
            match brackets.keys().find(|x| **x == row[i]) {
                Some(_) => { stack.push(row[i]); }
                None    => {
                    if stack.len() == 0 { panic!(); }
                    let rb = stack.pop().unwrap();

                    // Check if invalid bracket used
                    if brackets[&rb] != row[i] {
                        tot += br_val[&row[i]];
                        break;
                    }
                }
            }
        }
    }
    println!("{}", tot);
}

fn read_lines<P>(filename: P) -> Vec<Vec<char>> where P: AsRef<Path> {
    // Converts input lines to 2D vector of char values
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    return lines
    .map(|l| l.unwrap().chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
}
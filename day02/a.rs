use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let commands = read_lines("input.txt");

    // Change position and depth by command value
    let mut pos = 0;
    let mut depth = 0;

    for (name, num) in commands {
        match name.as_str() {
            "forward" => { pos += num; },
            "down" => { depth += num; },
            "up" => { depth -= num; },
            _ => { panic!(); }
        }
    }

    println!("{}", pos * depth);
}

fn read_lines<P>(filename: P) -> Vec<(String, i32)> where P: AsRef<Path> {
    // Converts input lines to vector of string, int pairs
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut commands = vec![];
    for line in lines {
        let text = line.unwrap();
        let v = text.split_whitespace().collect::<Vec<_>>();
        commands.push((v[0].to_string(), v[1].parse::<i32>().unwrap()));
    }
    return commands;
}
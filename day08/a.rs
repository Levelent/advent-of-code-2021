use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let sizes = read_lines("input.txt");

    let uniques = vec![2, 3, 4, 7];
    let total = sizes.iter().filter(|s| uniques.contains(&s)).count();

    println!("{}", total);
}

fn read_lines<P>(filename: P) -> Vec<usize> where P: AsRef<Path> {
    // Converts each test digit into it's length, and returns as a single vector
    
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut configs = vec![];
    for line in lines {
        let text = line.unwrap();
        let mut iter = text.split("|");

        iter.next();

        let second = iter.next().unwrap()
        .split_whitespace()
        .map(|x| x.len())
        .collect::<Vec<_>>();

        configs.push(second);
    }
    return configs.into_iter().flatten().collect::<Vec<_>>();
}
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let test_configs = read_lines("input.txt");

    // Known digit if it has the following length
    let uniques = vec![2, 3, 4, 7];

    // We only care about the lengths across all test cases
    let sizes = test_configs.into_iter()
    .flatten()
    .map(|x| x.len())
    .collect::<Vec<_>>();

    // Count only those having the desired length
    let total = sizes.iter().filter(|s| uniques.contains(&s)).count();
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> Vec<Vec<String>> where P: AsRef<Path> {
    // Converts each test set into string vectors, returning a vector of test sets
    
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut configs = vec![];
    for line in lines {
        let text = line.unwrap();
        let mut iter = text.split("|");

        iter.next();

        let second = iter.next().unwrap()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

        configs.push(second);
    }
    return configs;
}
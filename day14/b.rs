use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

fn main() -> () {
    let (template, rules) = read_lines("input.txt");

    // Split into pairs of adjacent letters
    let mut pairs : HashMap<Vec<char>, usize> = HashMap::new();
    for i in 0..template.len()-1 {
        *pairs.entry(template[i..i+2].to_vec()).or_default() += 1;
    }

    // Counts of each letter
    let mut counts : HashMap<_, usize> = HashMap::new();
    for i in 0..template.len() {
        *counts.entry(template[i]).or_default() += 1;
    }
    
    // Run simulation
    for _ in 0..40 {
        let mut new_pairs = pairs.clone();
        let mut new_counts = counts;
        for (pair, freq) in pairs {
            if rules.contains_key(&pair) {
                // Add all copies of the new letter to counts
                *new_counts.entry(rules[&pair]).or_default() += freq;
                // Add the new left and right two-character pairings
                *new_pairs.entry(vec![pair[0], rules[&pair]]).or_default() += freq;
                *new_pairs.entry(vec![rules[&pair], pair[1]]).or_default() += freq;
                // Remove the old two-character pairing
                *new_pairs.entry(pair).or_default() -= freq;
            }
        }
        pairs = new_pairs;
        counts = new_counts;
    }

    println!("{}", counts.values().max().unwrap() - counts.values().min().unwrap());
}

fn read_lines<P>(filename: P) -> (Vec<char>, HashMap<Vec<char>, char>) where P: AsRef<Path> {
    // Converts input lines
    let file = File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    // Convert initial template to a character vector
    let template : Vec<char> = lines.next().unwrap().unwrap().chars().collect();

    lines.next(); // Ignore the blank line

    // Split rules into left and right parts, then convert to hashmap
    let rules = lines.map(
        |l| l.unwrap()
        .split_whitespace()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>()
    ).map(
        |v| (
            v[0].chars().collect::<Vec<_>>(), 
            v[2].chars().next().unwrap()
        )
    ).collect::<HashMap<_,_>>();

    return (template, rules);
}
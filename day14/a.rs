use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

fn main() -> () {
    let (mut template, rules) = read_lines("input.txt");

    // Run naive simulation
    for _ in 0..10 {
        // Take all two-character slices, and append to new string
        let mut new_template = vec![];
        for i in 0..template.len()-1 {
            let comp = &template[i..i+2];
            new_template.push(template[i]); // Push current letter
            if rules.contains_key(comp) {
                new_template.push(rules[comp]); // Push new middle letter, if needed
            }
        }
        new_template.push(template[template.len()-1]); // Push last letter
        template = new_template;
    }

    // Get frequency hashmap of characters
    let freqs : HashMap<&char, usize> = template.iter()
    .fold(
        HashMap::new(), |mut m, c| {*m.entry(c).or_default() += 1; m}
    );

    println!("{}", freqs.values().max().unwrap() - freqs.values().min().unwrap());
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
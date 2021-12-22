use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::{HashSet, HashMap};

fn main() -> () {
    let configurations = read_lines("input.txt");

    // Known mappings
    let unique_map = HashMap::from([
        (2, 1), (3, 7), (4, 4), (7, 8)
    ]);

    let mut total = 0;
    for (examples, tests) in configurations {
        let mut assignments = vec![HashSet::new(); 10];
        for example in examples {
            let len = example.len();
            // Apply known mappings
            match len {
                2 | 3 | 4 | 7 => { 
                    let idx = unique_map.get(&len).unwrap();
                    assignments[*idx] = example;
                },
                5 => {
                    if assignments[7].is_subset(&example) {
                        assignments[3] = example;
                    } else if assignments[4].difference(&assignments[1]).cloned().collect::<HashSet<_>>().is_subset(&example) {
                        assignments[5] = example;
                    } else {
                        assignments[2] = example;
                    }
                },
                6 => {
                    if assignments[4].is_subset(&example) {
                        assignments[9] = example;
                    } else if !assignments[1].is_subset(&example) {
                        assignments[6] = example;
                    } else {
                        assignments[0] = example;
                    }
                },
                _ => ()
            }
        }
        // println!("{:?}", assignments);
        let mut str_val = String::with_capacity(4);
        for test in tests {
            let idx = assignments.iter().position(|s| s.eq(&test)).unwrap();
            str_val.push_str(&idx.to_string());
        }
        total += str_val.parse::<i32>().unwrap();
    }
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> where P: AsRef<Path> {
    // Converts to a vector of character hashsets, with examples sorted by size
    
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut configs = vec![];
    for line in lines {
        let text = line.unwrap();
        let mut iter = text.split("|");

        let mut first = iter.next().unwrap()
        .split_whitespace()
        .map(|x| x.to_string().chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
        first.sort_by_key(|a| a.len());

        let second = iter.next().unwrap()
        .split_whitespace()
        .map(|x| x.to_string().chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

        configs.push((first, second));
    }
    return configs;
}
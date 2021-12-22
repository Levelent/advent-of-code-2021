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
    for (mut examples, tests) in configurations {
        
        let mut assignments = vec![HashSet::new(); 10];

        examples.sort_by_key(|s| s.len()); // guarantees certain digits appear before others
        
        for example in examples {
            let len = example.len();
            match len {
                // Apply mappings known via length
                2 | 3 | 4 | 7 => {
                    let idx = unique_map.get(&len).unwrap();
                    assignments[*idx] = example;
                },
                // Apply mappings known via digits 1, 4, 7 (guaranteed to exist at this point)
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

        // Create string from test digits, convert to int
        total += tests.iter()
        .map(
            |x| (assignments.iter()
            .position(|s| s.eq(x))
            .unwrap() as u8 + 48) as char // 48 is ASCII for '0'  
        )
        .collect::<String>()
        .parse::<i32>().unwrap();
    }
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> where P: AsRef<Path> {
    // Converts to a vector of character hashsets split into example and test sets
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut configs = vec![];
    for line in lines {
        let text = line.unwrap();
        let mut iter = text.split("|");

        let first = iter.next().unwrap()
        .split_whitespace()
        .map(|x| x.to_string().chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

        let second = iter.next().unwrap()
        .split_whitespace()
        .map(|x| x.to_string().chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

        configs.push((first, second));
    }
    return configs;
}
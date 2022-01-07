use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let (mut coords, folds) = read_lines("input.txt");

    let mut first = true;
    for (vert, offset) in folds {
        // Fold the sheet, moving necessary coordinates
        for i in 0..coords.len() {
            // pivot if above offset
            let pos = coords[i][vert as usize];
            if pos > offset {
                // pos - 2 * (pos - offset)
                let new_pos = offset + offset - pos;
                if new_pos >= 0 {
                    coords[i][vert as usize] = new_pos; 
                } else {  // Folded outside bounds, so mark
                    coords[i] = vec![-1, -1];
                }
            }
        }
        
        // Remove marked and duplicate coordinates
        coords.sort();
        coords.dedup();
        coords.retain(|x| *x != vec![-1, -1]);

        // Print out remaining num of dots after first fold
        if first {
            println!("Part 1: {}", coords.len());
            first = false;
        }
    }

    // Print out folded grid
    let mut v = vec![vec!['.'; 39]; 6];
    for coord in &coords {
        v[coord[1] as usize][coord[0] as usize] = '█';
    }

    println!("Part 2:");
    for row in v {
        println!("{}", row.iter().collect::<String>());
    }
}

fn read_lines<P>(filename: P) -> (Vec<Vec<i32>>, Vec<(bool, i32)>) where P: AsRef<Path> {
    // Converts input lines into coordinates and folds
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    let text : Vec<_> = lines.map(|l| l.unwrap()).collect();
    let groups: Vec<_> = text.split(String::is_empty).collect();
    
    // Convert coord strings into vector of 2-element integer vectors
    let coords = groups[0].iter().map(
        |s| s.split(",").map(
            |x| x.parse::<i32>().unwrap()
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    // Get last word, split by the '='. LHS to bool, RHS to integer
    let folds = groups[1].iter().map(
        |s| s.split_whitespace().collect::<Vec<_>>()
    ).map(
        |s| s[2].split("=").collect::<Vec<_>>()
    ).map(
        |v| (v[0] == "y", v[1].parse::<i32>().unwrap())
    ).collect::<Vec<_>>();

    return (coords, folds);
}
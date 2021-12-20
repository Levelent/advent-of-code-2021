use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;
use std::cmp;

fn main() -> () {
    let coord_pairs = read_lines("input.txt");

    let mut seen_points = HashSet::new();
    let mut seen_twice = HashSet::new();
    
    // Determine points covered by each line, try to add to hashset
    // If adding fails, must have seen more than once
    // To avoid double counting, another hashset is used

    for pair in coord_pairs {
        for point in points_connecting(pair[0], pair[1], pair[2], pair[3]) {
            if !seen_points.insert(point) {
                seen_twice.insert(point);
            }
        }
    }

    println!("{}", seen_twice.len());
}

fn points_connecting(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    // Determine the number of squares travelled
    let x_dist = x2 - x1;
    let y_dist = y2 - y1;
    let abs_dist = cmp::max(x_dist.abs(), y_dist.abs());

    // Avoid division by 0 if coordinates equal
    if abs_dist == 0 { return vec![(x1, y1)]; }
    let mut points = vec![];

    // Determine step direction
    let x_dir = x_dist / abs_dist;
    let y_dir = y_dist / abs_dist;

    // Apply step direction each iteration
    for i in 0..=abs_dist {
        points.push((x1 + x_dir * i, y1 + y_dir * i));
    }
    return points;
}

fn read_lines<P>(filename: P) -> Vec<Vec<i32>> where P: AsRef<Path> {
    // Converts input lines to vectors representing coordinate pairs
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut coord_pairs = vec![];
    for line in lines {
        // Get each line into 4 comma-separated integers
        let cleaned_line = line.unwrap()
        .replace("->", ",")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

        // Convert to vector of i32s
        let coord_pair = cleaned_line
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

        coord_pairs.push(coord_pair);
    }

    return coord_pairs;
}
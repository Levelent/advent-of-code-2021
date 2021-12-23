use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let mut num_grid = read_lines("input.txt");

    let mut flashes = 0;
    let mut iter = 0;

    // Add one to each square, start a flash chain if it hits 10
    // Keep going until all squares flash simultaneously
    while flashes < 100 {
        flashes = 0;
        for i in 0..num_grid.len() {
            for j in 0..num_grid[0].len() {
                flashes += adjacent_flashes(&mut num_grid, i, j);
            }
        }
        reset_flashed(&mut num_grid);
        iter += 1;
    }
    
    println!("{}", iter);
}

fn adjacent_flashes(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    grid[row][col] += 1;  // Increment energy level
    if grid[row][col] != 10 { return 0; }  // Avoid double counting flashes

    // Determine total of all flashes in the chain caused by this flash
    return 1 + determine_adjacent(grid.len(), grid[0].len(), row, col)
    .iter()
    .map(|(r, c)| adjacent_flashes(grid, *r, *c))
    .fold(0, |acc, x| acc + x);
}

fn determine_adjacent(num_rows: usize, num_cols: usize, mut row: usize, mut col: usize) -> Vec<(usize, usize)> {
    // Convert to 1-indexed to avoid going out of usize bounds
    row += 1;
    col += 1;

    // Determine which of adjacent 8 directions exist, and return their row/col positions
    let mut adj = vec![];
    for r in row-1..=row+1 {
        for c in col-1..=col+1 {
            if r <= 0 || r > num_rows || c <= 0 || c > num_cols || (r == row && c == col) {
                continue;
            }
            // Convert back to 0-indexed
            adj.push((r - 1, c - 1));
        }
    }
    return adj;
}

fn reset_func(num: u8) -> u8 {
    // If flashed square, reset to 0
    if num > 9 { return 0; }
    return num;
}

fn reset_flashed(grid: &mut Vec<Vec<u8>>) {
    // Apply reset to all flashed squares during this iteration
    *grid = grid.iter()
    .map(
        |r| r.iter()
        .map(|c| reset_func(*c))
        .collect::<Vec<_>>()
    )
    .collect::<Vec<_>>();
}

fn read_lines<P>(filename: P) -> Vec<Vec<u8>> where P: AsRef<Path> {
    // Converts input lines to 2D vector of integers
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    return lines
    .map(
        |l| l.unwrap()
        .chars()
        .map(|x| x as u8 - 48) // 48 is '0' in ASCII
        .collect::<Vec<_>>()
    )
    .collect::<Vec<_>>();
}
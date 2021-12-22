use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let num_grid = read_lines("input.txt");

    // Add square if it is smaller than all adjacent squares
    let mut total = 0;
    for i in 0..num_grid.len() {
        for j in 0..num_grid[0].len() {
            if num_grid[i][j] < min_adjacent_nums(&num_grid, i, j) {
                total += (num_grid[i][j] + 1) as usize;
            }
        }
    }
    println!("{}", total);
}

fn min_adjacent_nums(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> u8 {
    // The minimum of all adjacent square values
    let mut v = vec![];
    if row > 0 { v.push(grid[row-1][col]); }
    if row < grid.len() - 1 { v.push(grid[row+1][col]); }
    if col > 0 { v.push(grid[row][col-1]); }
    if col < grid[0].len() - 1 { v.push(grid[row][col+1]); }
    return *v.iter().min().unwrap();
}

fn read_lines<P>(filename: P) -> Vec<Vec<u8>> where P: AsRef<Path> {
    // Converts input lines to 2D vector of bool values
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    // let mut num_grid = vec![];
    return lines
    .map(
        |l| l.unwrap()
        .chars()
        .map(|x| x as u8 - 48) // 48 is '0' in ASCII
        .collect::<Vec<_>>()
    )
    .collect::<Vec<_>>();
}
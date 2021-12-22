use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let mut num_grid = read_lines("input.txt");

    // Explore basin when encountering first element in it, then cover with 9s
    let mut basin_sizes = vec![];
    for i in 0..num_grid.len() {
        for j in 0..num_grid[0].len() {
            if num_grid[i][j] != 9 { // Unexplored basin
                basin_sizes.push(basin_size(&mut num_grid, i, j));
            }
        }
    }

    // Take three largest basin sizes
    basin_sizes.sort();
    basin_sizes.reverse();
    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

fn basin_size(grid: &mut Vec<Vec<usize>>, row: usize, col: usize) -> usize {
    // Determine the size of the basin containing this cell, and cover it

    if grid[row][col] == 9 { return 0; }  // Boundary or already included
    let mut total = 1;  // Count self
    grid[row][col] = 9;  // Prevent double-counting
    if row > 0 {  // Explore to the left
        total += basin_size(grid, row-1, col); 
    }
    if row < grid.len() - 1 {  // Explore to the right
        total += basin_size(grid, row+1, col); 
    }
    if col > 0 {  // Explore upwards
        total += basin_size(grid, row, col-1); 
    }
    if col < grid[0].len() - 1 {  // Explore downwards
        total += basin_size(grid, row, col+1);
    }
    return total;
}

fn read_lines<P>(filename: P) -> Vec<Vec<usize>> where P: AsRef<Path> {
    // Converts input lines to 2D vector of positive integers
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    // let mut num_grid = vec![];
    return lines
    .map(
        |l| l.unwrap()
        .chars()
        .map(|x| (x as u8 - 48) as usize) // 48 is '0' in ASCII
        .collect::<Vec<_>>()
    )
    .collect::<Vec<_>>();
}
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let mut vec_max = read_lines("input.txt");
    let mut vec_min = vec_max.clone();

    // Find bit vector of Oxygen Generator Rating
    let mut freq_max = 0;

    for j in 0..vec_max[0].len() {
        for row in &vec_max { freq_max += row[j] as usize; }

        // Ensures that if on a boundary, true is chosen
        let most_common_max = freq_max >= (vec_max.len() + 1) / 2;
        
        vec_max.retain(|x| x[j] == most_common_max);

        if vec_max.len() == 1 { break; }  // Found only answer
        freq_max = 0;
    }

    // Find bit vector of CO2 Scrubber Rating
    let mut freq_min = 0;

    for j in 0..vec_min[0].len() {
        for row in &vec_min { freq_min += row[j] as usize; }

        // Ensures that if on a boundary, false is chosen
        let least_common_min = freq_min < (vec_min.len() + 1) / 2;

        vec_min.retain(|x| x[j] == least_common_min);

        if vec_min.len() == 1 { break; }  // Found only answer
        freq_min = 0;
    }

    // Calculate final total
    let val_min = bitvec_to_usize(&vec_min[0]);
    let val_max = bitvec_to_usize(&vec_max[0]);
    println!("{}", val_max * val_min);
    
}

fn bitvec_to_usize(bitvec: &Vec<bool>) -> usize {
    // Generates integer from binary bit vector
    let mut val = 0;
    for b in bitvec {
        if *b { val = val * 2 + 1; } 
        else { val *= 2; }
    }
    return val;
}

fn read_lines<P>(filename: P) -> Vec<Vec<bool>> where P: AsRef<Path> {
    // Converts input lines to 2D vector of bool values
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    let mut vec_max = vec![];
    for line in lines {
        let mut bools = vec![];
        if let Ok(bitstring) = line {
            for bit in bitstring.chars() {
                bools.push(bit == '1');
            }
        }
        vec_max.push(bools);
    }
    return vec_max;
}
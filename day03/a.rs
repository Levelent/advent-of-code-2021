use std::io::{self, BufRead};
// use io::Stdin;
use std::fs::File;
use std::path::Path;

fn main() -> () {
    let bools_vec = read_lines("input.txt");
    // println!("{:?}", bools_vec);

    let mut max_vec = vec![];
    for i in 0..bools_vec[0].len() {
        
        let mut freq = 0;
        for j in 0..bools_vec.len() {
            if bools_vec[j][i] {
                freq += 1;
            }
        }
        max_vec.push(freq > bools_vec.len() / 2);
    }
    // println!("{:?}", max_vec);
    let mut val = 0;
    let mut neg_val = 0;
    for b in max_vec {
        match b {
            true => { val = val * 2 + 1; neg_val *= 2; }
            false => { val *= 2; neg_val = neg_val * 2 + 1; }
        }
    }
    // println!("{}", val);
    // println!("{}", neg_val);
    println!("{}", val * neg_val);
    

    // Change position and depth by given value, depending on command
    // for _ in 0..1000 {
    //     let (cmd, num) = get_next_command(&stdin);
    //     if cmd == "forward" {
    //         pos += num;
    //     } else if cmd == "down" {
    //         depth += num;
    //     } else if cmd == "up" {
    //         depth -= num;
    //     } else {
    //         panic!();
    //     }
    // }
    // // println!("Pos: {}, Depth: {}", pos, depth);
    // println!("{}", pos * depth);
}

fn read_lines<P>(filename: P) -> Vec<Vec<bool>> where P: AsRef<Path> {
    let file = File::open(filename).expect("Oh no");
    let lines = io::BufReader::new(file).lines();

    let mut bools_vec = vec![];
    for line in lines {
        let mut bools = vec![];
        if let Ok(bitstring) = line {
            // println!("Got string {}", bitstring);
            for bit in bitstring.chars() {
                bools.push(bit == '1');
            }
        }
        bools_vec.push(bools);
    }
    return bools_vec;
}
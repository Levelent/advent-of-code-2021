use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::process;

fn main() -> () {
    let (draws, mut bingo_cards) = read_lines("input.txt");

    for draw in draws {
        for mut card in &mut bingo_cards {
            modify_card(&mut card, draw);

            // Calculate total and exit upon first bingo
            if check_bingo(&card) {
                println!("{}", draw * unmarked_total(&card));
                process::exit(0);
            }
        }
    }
}

fn modify_card(card: &mut Vec<Vec<i32>>, target: i32) {
    for i in 0..card.len() {
        for j in 0..card[0].len() {
            if target == card[i][j] {
                card[i][j] = -1;
            }
        }
    }
}

fn check_bingo(card: &Vec<Vec<i32>>) -> bool {
    for i in 0..card.len() {
        if card[i].iter().filter(|x| **x == -1).count() == 5 {
            return true;
        }
    }
    for j in 0..card[0].len() {
        let mut result = true;
        for i in 0..card.len() {
            result &= card[i][j] == -1;
        }
        if result { return true; }
    }
    return false;
}

fn unmarked_total(card: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for row in card {
        for num in row {
            if *num != -1 {
                total += num;
            }
        }
    }
    return total;
}

fn read_lines<P>(filename: P) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) where P: AsRef<Path> {
    // Converts input lines to a vector of integers, and a vector of 2D vectors.
    let file = File::open(filename).unwrap();
    let mut buf_reader = io::BufReader::new(file);

    let mut buf = String::new();
    buf_reader.read_line(&mut buf).unwrap();

    // Convert untrimmed first line to vector of integers
    let mut called_nums = vec![];
    for elem in buf.split(",") {
        called_nums.push(elem.trim().parse::<i32>().unwrap());
    }

    // Alternative approach
    // let mut called_nums = buf.split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<_>>();

    // Vector of all remaining non-blank lines
    let lines = buf_reader.lines()
    .map(|l| l.unwrap())
    .filter(|l| l != "")
    .collect::<Vec<_>>();
    
    // Divide remaining lines into 5 x 5 2D vector grids
    let mut cards = vec![];
    for num_cards in 0..(lines.len() / 5) {

        let mut card = vec![];
        for num_row in 0..5 {

            let line = &lines[num_cards * 5 + num_row];
            
            // Convert line to vector of integers
            let card_row = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

            card.push(card_row);
        }
        cards.push(card);
    }

    return (called_nums, cards);
}
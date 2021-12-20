use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::process;

fn main() -> () {
    let (draws, mut bingo_cards) = read_lines("input.txt");

    for draw in draws {
        for mut card in &mut bingo_cards {
            modify_card(&mut card, draw);
        }

        // Filter out all cards that have bingo'ed this call
        let new_cards = bingo_cards.clone().into_iter()
        .filter(|card| !check_bingo(&card))
        .collect::<Vec<_>>();

        // If all cards have bingo'ed, get the total of the only card remaining.
        if new_cards.is_empty() {
            println!("{}", draw * unmarked_total(&bingo_cards[0]));
            process::exit(0);
        }

        bingo_cards = new_cards;
    }
}

fn modify_card(card: &mut Vec<Vec<i32>>, target: i32) {
    // Marks called squares with -1
    for i in 0..card.len() {
        for j in 0..card[0].len() {
            if target == card[i][j] {
                card[i][j] = -1;
            }
        }
    }
}

fn check_bingo(card: &Vec<Vec<i32>>) -> bool {
    // Checks rows
    for row in card {
        if row.iter().filter(|x| **x == -1).count() == row.len() {
            return true;
        }
    }

    // Checks columns
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
    // Sums the unmarked numbers on the card
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
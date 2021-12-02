use std::io;
use io::Stdin;

fn main() -> () {
    let stdin = io::stdin();

    // Counting # of times current line value greater than last
    let mut prev = get_next_int(&stdin);
    let mut tot = 0;

    for _ in 0..1999 {  // Remove 1 from number of lines
        let curr = get_next_int(&stdin);
        if curr > prev {
            tot += 1;
        }
        prev = curr;
    }

    println!("{}", tot);
}

fn get_next_int(stdin: &Stdin) -> i32 {
    let mut text = String::new();
    stdin.read_line(&mut text).expect("Oh no");
    // println!("Got string {}", text);
    let num = text.trim().parse::<i32>().unwrap();
    // println!("Got num {}", num);
    return num;
}
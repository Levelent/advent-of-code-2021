use std::io;
use io::Stdin;

fn main() -> () {
    let stdin = io::stdin();

    // Counting # of times most recent value greater than 4th most recent
    let mut prev3 = get_next_int(&stdin);
    let mut prev2 = get_next_int(&stdin);
    let mut prev1 = get_next_int(&stdin);
    let mut tot = 0;

    for _ in 0..1997 {  // Remove 3 from number of lines
        let curr = get_next_int(&stdin);
        if curr > prev3 {
            tot += 1;
        }
        prev3 = prev2;
        prev2 = prev1;
        prev1 = curr;
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
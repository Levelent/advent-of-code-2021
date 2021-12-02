use std::io;
use io::Stdin;

fn main() -> () {
    let stdin = io::stdin();

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    // Change position and depth by given value, depending on command
    for _ in 0..1000 {
        let (cmd, num) = get_next_command(&stdin);
        if cmd == "forward" {
            pos += num;
            depth += aim * num;
        } else if cmd == "down" {
            aim += num;
        } else if cmd == "up" {
            aim -= num;
        } else {
            panic!();
        }
        // println!("{}, {}, {}", pos, depth, aim);
    }
    println!("{}", pos * depth);
}

fn get_next_command(stdin: &Stdin) -> (String, i32) {
    let mut text = String::new();
    stdin.read_line(&mut text).expect("Oh no");
    // println!("Got string {}", text);
    let v = text.trim().split(" ").collect::<Vec<&str>>();
    
    let num = v[1].parse::<i32>().unwrap();
    // println!("Got num {}", num);
    return (v[0].to_string(), num);
}
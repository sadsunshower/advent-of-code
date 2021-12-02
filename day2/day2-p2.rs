use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut tok = line.split(" ");
        let command = tok.next().unwrap();
        let amount: i32 = tok.next().unwrap().parse().unwrap();

        match command {
            "forward" => { x += amount; y += aim * amount },
            "up" => { aim -= amount },
            "down" => { aim += amount },
            _ => continue
        }
    };

    println!("Final position: {:?}, {:?} multiplies to {:?}", x, y, x * y)
}
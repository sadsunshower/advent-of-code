use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();

    let mut x = 0;
    let mut y = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut tok = line.split(" ");
        let command = tok.next().unwrap();
        let amount: i32 = tok.next().unwrap().parse().unwrap();

        match command {
            "forward" => x += amount,
            "up" => y -= amount,
            "down" => y += amount,
            _ => continue
        }
    };

    println!("Final position: {:?}, {:?} multiplies to {:?}", x, y, x * y)
}
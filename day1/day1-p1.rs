use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();

    let mut previous: Option<i32> = None;
    let mut increases = 0;

    for line in stdin.lock().lines() {
        let curr: i32 = line.unwrap().parse().unwrap();

        if previous.is_some() && (curr > previous.unwrap()) {
            increases += 1;
        };

        previous = Some(curr)
    };

    println!("Increases {:?} times", increases)
}
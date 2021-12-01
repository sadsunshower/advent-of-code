use std::io::{BufRead};

const SIZE: usize = 3;

fn main() {
    let stdin = std::io::stdin();

    let mut window: [i32; SIZE] = [0, 0, 0];
    let mut previous = 0;
    let mut init = 0;
    let mut increases = 0;

    for line in stdin.lock().lines() {
        let curr: i32 = line.unwrap().parse().unwrap();

        if init < SIZE {
            window[init] = curr;
            previous += curr;
            init += 1
        } else {
            let next = previous - window[0] + curr;
            window.rotate_left(1);
            window[SIZE - 1] = curr;

            if next > previous {
                increases += 1
            };

            previous = next;
        };
    };

    println!("Increases {:?} times", increases)
}
use std::io::{BufRead};

const SIZE: usize = 12;

fn main() {
    let stdin = std::io::stdin();

    // Boyer-Moore style trick
    // https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm
    let mut common: [i32; SIZE] = [0; SIZE];

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        assert!(line.len() == SIZE);

        for pos in 0 .. SIZE {
            match line.chars().nth(pos) {
                Some('0') => common[pos] += 1,
                Some('1') => common[pos] -= 1,
                _ => continue
            }
        }
    };

    let mut gamma = 0;

    for bit in 0 .. SIZE {
        assert!(common[bit] != 0);

        if common[bit] < 0 {
            gamma = (gamma << 1) | 1
        } else {
            gamma = gamma << 1
        }
    }

    let epsilon = !gamma & ((1 << SIZE) - 1);

    println!("Epsilon: {:b}, Gamma: {:b}, Product: {:?}", epsilon, gamma, epsilon * gamma)
}
use std::io::{BufRead};

fn solve(numbers: &Vec<String>, oxygen: bool, position: usize) -> String {
    assert!(numbers.len() != 0);

    if numbers.len() == 1 {
        return numbers.first().unwrap().to_string();
    };

    // Boyer-Moore style trick
    // https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm
    let mut common = 0;

    for number in numbers {
        match number.chars().nth(position) {
            Some('0') => common += 1,
            Some('1') => common -= 1,
            _ => continue
        }
    };

    let mut next: Vec<String> = Vec::new();
    let chosen = if oxygen {
        if common > 0 { '0' } else { '1' }
    } else {
        if common > 0 { '1' } else { '0' }
    };

    for number in numbers {
        match number.chars().nth(position) {
            Some(c) => if c == chosen { next.push(number.to_string()) },
            _ => continue
        }
    };

    assert!(next.len() != 0);

    solve(&next, oxygen, position + 1)
}

fn main() {
    let stdin = std::io::stdin();
    let numbers: Vec<String> = stdin.lock().lines().map(|r| r.unwrap()).collect();

    let oxygen_bits = solve(&numbers, true, 0);
    let carbon_bits = solve(&numbers, false, 0);

    assert!(oxygen_bits.len() == carbon_bits.len());

    let mut oxygen = 0;
    let mut carbon = 0;

    for bit in 0 .. oxygen_bits.len() {
        oxygen = match oxygen_bits.chars().nth(bit) {
            Some('0') => oxygen << 1,
            Some('1') => (oxygen << 1) | 1,
            _ => oxygen
        };

        carbon = match carbon_bits.chars().nth(bit) {
            Some('0') => carbon << 1,
            Some('1') => (carbon << 1) | 1,
            _ => carbon
        };
    }

    println!("Oxygen: {:b}, CO2: {:b}, Product: {:?}", oxygen, carbon, oxygen * carbon)
}
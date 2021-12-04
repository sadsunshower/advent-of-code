use std::io::{BufRead};

const SIZE: usize = 5;
const EARLY_EXIT: bool = false; // Set to true for part 1

#[derive(Debug)]
struct Board {
    board: [[Option<i32>; SIZE]; SIZE],
    row_counter: [i32; SIZE],
    col_counter: [i32; SIZE],
    unmarked: i32,
    won: bool,
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let numbers_line = lines.next().unwrap().unwrap();
    let numbers = numbers_line.split(",");

    let mut boards: Vec<Board> = Vec::new();

    let mut next = Board {
        board: [[None; SIZE]; SIZE],
        row_counter: [SIZE as i32; SIZE],
        col_counter: [SIZE as i32; SIZE],
        unmarked: 0,
        won: false,
    };
    let mut row_position = 0;

    for line in lines {
        let line = line.unwrap();

        if line.len() == 0 {
            continue
        };

        let row = line.split(" ");
        let mut col_position = 0;

        for number in row {
            if number.len() == 0 {
                continue
            };

            assert!(col_position < SIZE);

            let number = number.parse().unwrap();
            next.unmarked += number;
            next.board[row_position][col_position] = Some(number);
            col_position += 1;
        }

        assert!(col_position == SIZE);

        row_position += 1;

        if row_position == SIZE {
            boards.push(next);

            next = Board {
                board: [[None; SIZE]; SIZE],
                row_counter: [SIZE as i32; SIZE],
                col_counter: [SIZE as i32; SIZE],
                unmarked: 0,
                won: false,
            };
            row_position = 0
        }
    };

    'game: for number in numbers {
        let number = number.parse().unwrap();

        for b in 0 .. boards.len() {
            let mut board = &mut boards[b];

            if board.won {
                continue
            }

            'scan: for row in 0 .. SIZE {
                for col in 0 .. SIZE {
                    if board.board[row][col].is_some() && board.board[row][col].unwrap() == number {
                        board.unmarked -= number;
                        board.row_counter[row] -= 1;
                        board.col_counter[col] -= 1;
                        board.board[row][col] = None;
                    };

                    if board.row_counter[row] == 0 || board.col_counter[col] == 0 {
                        println!("Board {:?} wins with score {:?}", (b + 1), board.unmarked * number);
                        board.won = true;
                        if EARLY_EXIT {
                            break 'game
                        } else {
                            break 'scan
                        }
                    }
                }
            }
        }
    }
}
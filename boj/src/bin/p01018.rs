use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn required_changes(board: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut black_start: usize = 0;
    for i in x..x + 8 {
        for j in y..y + 8 {
            print!("{}", board[i][j]);
            if matches!(
                (i % 2, j % 2, board[i][j]),
                (0, 0, 'B') | (0, 1, 'W') | (1, 0, 'W') | (1, 1, 'B')
            ) {
                black_start += 1;
            }
        }
    }

    black_start.min(64 - black_start)
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.split_ascii_whitespace();
    let mut output = String::new();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let board: Vec<Vec<char>> = input.map(|l| l.chars().collect::<Vec<char>>()).collect();

    let mut min_changes: usize = 0;
    for i in 0..height - 8 {
        for j in 0..width - 8 {
            min_changes = min_changes.min(required_changes(&board, i, j))
        }
    }

    write!(output, "{min_changes}").unwrap();
    print!("{output}");
}

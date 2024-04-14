use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn find_min_moves(target: usize, broken: &[u8]) -> usize {
    if target == 100 {
        return 0;
    }

    let direct_moves = target.abs_diff(100);

    let is_usable = |n: usize| -> bool {
        if n == 0 {
            return !broken.contains(&0);
        }

        let mut num = n;
        while num > 0 {
            let digit = (num % 10) as u8;
            if broken.contains(&digit) {
                return false;
            }
            num /= 10;
        }
        true
    };

    let mut min_moves = direct_moves;
    let num_range = if target > 100 {
        0..=target * 2
    } else {
        0..=1000
    };
    for n in num_range {
        if is_usable(n) {
            let num_moves = n.abs_diff(target) + n.to_string().len();
            min_moves = min_moves.min(num_moves);
        }
    }

    min_moves
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let target = parse_next();
    let num_broken = parse_next();
    let broken = (0..num_broken)
        .map(|_| parse_next() as u8)
        .collect::<Vec<u8>>();

    write!(output, "{}", find_min_moves(target, &broken)).unwrap();
    print!("{output}");
}

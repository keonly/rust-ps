use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn count_occurrences(n: usize, s: &str) -> usize {
    let mut remaining = 2 * n + 1;
    let mut count = 0;
    let mut start = 0;

    for (i, c) in s.chars().enumerate() {
        let pattern = if (i - start) % 2 == 0 { 'I' } else { 'O' };

        if c == pattern {
            remaining -= 1;
        } else {
            start = if c == 'I' { i } else { i + 1 };
            remaining = 2 * n + 1 - (c == 'I') as usize;
        }

        if remaining == 0 {
            count += 1;
            start += 2;
            remaining = 2;
        }
    }

    count
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let n = parse_next();
    let _ = input.next();
    let s = input.next().unwrap();

    write!(output, "{}", count_occurrences(n, s)).unwrap();
    print!("{output}");
}

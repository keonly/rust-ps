use std::collections::HashMap;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn count_squares(n: usize) -> usize {
    let squares: Vec<usize> = (1..).take_while(|x| x * x <= n).map(|x| x * x).collect();
    let mut counts: Vec<usize> = vec![4; n + 1];
    for s in squares.iter() {
        counts[*s] = 1;
    }

    for i in 1..=n {
        if counts[i] == 4 {
            counts[i] = squares
                .iter()
                .take_while(|x| **x <= i)
                .map(|x| counts[i - x])
                .min()
                .unwrap()
                + 1;
        }
    }

    counts[n]
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let n: usize = buffer.trim().parse::<usize>().unwrap();

    write!(output, "{}", count_squares(n)).unwrap();
    print!("{output}");
}

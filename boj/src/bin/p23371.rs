use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let [n, s, k] = [(); 3].map(|_| parse_next());
    let data: Vec<usize> = (0..k).map(|_| parse_next()).collect();
    let mut bins: Vec<usize> = vec![0; n];

    for x in data {
        bins[(x - 1) / s] += 1;
    }
    let max_height = *bins.iter().max().unwrap();

    for h in (1..=max_height).rev() {
        for i in 0..n {
            write!(output, "{}", if bins[i] >= h { '#' } else { '.' }).unwrap();
        }
        writeln!(output).unwrap();
    }
    for _ in 0..n {
        write!(output, "-").unwrap();
    }

    print!("{output}");
}

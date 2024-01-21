use std::fmt::Write;
use std::io::{stdin, Read};

fn hash_func(c: char, i: usize, R: u64, M: u64) -> u64 {
    let mut h: u64 = u64::from(c) - 96;
    (0..i).for_each(|_| h = (h * R) % M);
    h
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    stdin().read_line(&mut input).unwrap();

    let l: usize = input.trim().parse::<usize>().unwrap();
    const R: u64 = 31;
    const M: u64 = 1234567891;
    let mut powers: Vec<u64> = vec![0; l];
    let mut hash: u64 = 0;

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let hash: u64 = input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| hash_func(c, i, R, M))
        .sum();

    write!(output, "{}", hash % M).unwrap();
    print!("{}", output);
}


use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn josephus(n: usize, k: usize) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    let mut current: VecDeque<usize> = (1..=n).collect();

    for _ in 0..n {
        for _ in 0..(k - 1) {
            let temp: usize = current.pop_front().unwrap();
            current.push_back(temp);
        }
        result.push(current.pop_front().unwrap());
    }

    result
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let input: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let (n, k) = (input[0], input[1]);

    let josephus_seq: Vec<usize> = josephus(n, k);

    write!(
        output,
        "<{}>",
        josephus_seq
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
    .unwrap();
    print!("{output}");
}

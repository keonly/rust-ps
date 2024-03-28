use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn greedy(target: usize, coins: &[usize]) -> usize {
    coins
        .iter()
        .rev()
        .fold((0, target), |(count, leftover), val| {
            let take = leftover / val;
            (count + take, leftover - val * take)
        })
        .0
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let mut params = input.next().unwrap().split_ascii_whitespace();
    let [_, target] = [(); 2].map(|_| params.next().unwrap().parse::<usize>().unwrap());

    let coins: Vec<usize> = input.map(|c| c.parse::<usize>().unwrap()).collect();
    write!(output, "{}", greedy(target, &coins)).unwrap();
    print!("{output}");
}

use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn simulate_cards(n: usize) -> usize {
    let mut cards: VecDeque<usize> = (1..=n).collect();

    while cards.len() > 1 {
        cards.pop_front();
        let temp: usize = cards.pop_front().unwrap();
        cards.push_back(temp);
    }

    cards[0]
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let n: usize = input.trim().parse::<usize>().unwrap();

    write!(output, "{}", simulate_cards(n)).unwrap();
    print!("{output}");
}

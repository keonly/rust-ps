use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let input = buffer.split_ascii_whitespace().skip(1);
    let mut output = String::new();

    let mut times: Vec<usize> = input.map(|c| c.parse::<usize>().unwrap()).collect();
    let count: usize = times.len();
    times.sort_unstable();

    let min_total_time: usize = times
        .iter()
        .enumerate()
        .fold(0, |acc, (ord, time)| acc + (count - ord) * time);
    write!(output, "{min_total_time}").unwrap();
    print!("{output}");
}

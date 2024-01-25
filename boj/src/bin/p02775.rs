use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn combination(n: usize, k: usize) -> usize {
    if k > n {
        0
    } else {
        (1..=k).fold(1, |acc, val| acc * (n + 1 - val) / val)
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let mut nums = input
        .split_ascii_whitespace()
        .map(|c| c.parse::<usize>().unwrap());

    let tc: usize = nums.next().unwrap();
    for i in 0..tc {
        let k: usize = nums.next().unwrap();
        let n: usize = nums.next().unwrap();

        write!(output, "{}\n", combination(n + k, k + 1)).unwrap();
    }

    print!("{}", output);
}

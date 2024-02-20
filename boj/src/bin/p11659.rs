use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.lines().skip(1);
    let mut output = String::new();

    let mut nums: VecDeque<usize> = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    nums.iter_mut().fold(0, |acc, val| {
        *val += acc;
        *val
    });
    nums.push_front(0_usize);

    for test in input {
        let mut vals = test.split_ascii_whitespace();
        let [i, j] = [(); 2].map(|_| vals.next().unwrap().parse::<usize>().unwrap());
        writeln!(output, "{}", nums[j] - nums[i - 1]).unwrap();
    }

    print!("{output}");
}

use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let total: usize = input.next().unwrap().parse::<usize>().unwrap();
    let mut nums: Vec<usize> = input
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    nums.sort();
    let cutoff: usize = (total as f64 * 0.15).round() as usize;
    let trimmed: &[usize] = &nums[cutoff..total - cutoff];
    let trimmed_mean =
        (trimmed.iter().sum::<usize>() as f64 / trimmed.len() as f64).round() as usize;

    write!(output, "{trimmed_mean}").unwrap();
    print!("{output}");
}

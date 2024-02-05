use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let nums: Vec<usize> = input
        .trim()
        .split_ascii_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    write!(
        output,
        "{}",
        (nums[2] + nums[0] - 2 * nums[1] - 1) / (nums[0] - nums[1])
    )
    .unwrap();
    print!("{output}");
}

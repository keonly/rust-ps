use std::fmt::Write;
use std::io::{stdin, Read};

fn gcd(n1: usize, n2: usize) -> usize {
    if n2 == 0 {
        n1
    } else {
        gcd(n2, n1 % n2)
    }
}

fn lcm(n1: usize, n2: usize) -> usize {
    n1 * (n2 / gcd(n1, n2))
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<usize> = input
        .trim()
        .split_ascii_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    let (gcd, lcm) = (gcd(nums[0], nums[1]), lcm(nums[0], nums[1]));
    write!(output, "{}\n{}", gcd, lcm).unwrap();
    print!("{}", output);
}

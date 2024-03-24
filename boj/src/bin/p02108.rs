use std::collections::HashMap;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn arithmetic_mean(nums: &[isize]) -> f64 {
    nums.iter().sum::<isize>() as f64 / nums.len() as f64
}

fn median(n: usize, nums: &[isize]) -> isize {
    nums[n / 2]
}

fn mode(nums: &[isize]) -> isize {
    let mut counts = HashMap::new();
    for &x in nums {
        *counts.entry(x).or_insert(0) += 1;
    }

    let max_count = counts.values().max().cloned().unwrap_or(0);
    let mut modes: Vec<_> = counts
        .into_iter()
        .filter(|&(_, count)| count == max_count)
        .collect();
    modes.sort_unstable();
    let mode = *modes.get(1).unwrap_or(&modes[0]);
    mode.0
}

fn range(nums: &[isize]) -> isize {
    let (mut min, mut max) = (isize::MAX, isize::MIN);
    nums.iter().for_each(|&x| {
        min = min.min(x);
        max = max.max(x);
    });
    max - min
}

fn round_with_zero_fix(x: f64) -> isize {
    let rounded = x.round();
    if rounded == -0.0 {
        0
    } else {
        rounded as isize
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse::<usize>().unwrap();
    let mut nums: Vec<isize> = input
        .map(|c| c.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    nums.sort_unstable();

    writeln!(output, "{}", round_with_zero_fix(arithmetic_mean(&nums))).unwrap();
    writeln!(output, "{}", median(n, &nums)).unwrap();
    writeln!(output, "{}", mode(&nums)).unwrap();
    writeln!(output, "{}", range(&nums)).unwrap();
    print!("{output}");
}

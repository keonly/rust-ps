use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let nums: Vec<usize> = buffer
        .trim()
        .lines()
        .skip(1)
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let max_num: usize = *nums.iter().max().unwrap();

    let mut dp: Vec<usize> = vec![0; max_num + 1];
    dp[1] = 1;
    dp[2] = 2;
    dp[3] = 4;
    
    for i in 4..=max_num {
        dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
    }

    for n in nums.iter() {
        writeln!(output, "{}", dp[*n]).unwrap();
    }
    print!("{output}");
}

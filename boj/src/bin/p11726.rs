use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let n: usize = input.trim().parse::<usize>().unwrap();
    let mut dp: Vec<usize> = vec![0; 3.max(n + 1)];
    dp[1] = 1;
    dp[2] = 2;

    for i in 3..=n {
        dp[i] = (dp[i - 1] + dp[i - 2]) % 10007;
    }

    write!(output, "{}", dp[n]).unwrap();
    print!("{output}");
}

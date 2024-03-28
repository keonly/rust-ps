use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let n: usize = buffer.trim().parse::<usize>().unwrap();
    let mut dp: Vec<usize> = vec![0; 2.max(n)];
    dp[0] = 1;
    dp[1] = 3;

    for i in 2..n {
        dp[i] = (dp[i - 1] + 2 * dp[i - 2]) % 10007;
    }
    write!(output, "{}", dp[n - 1]).unwrap();
    print!("{output}");
}

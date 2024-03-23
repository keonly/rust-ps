use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let n: usize = buffer.trim().parse::<usize>().unwrap();
    let mut dp: Vec<isize> = vec![-1; n + 1];
    dp[3] = 1;
    if n >= 5 {
        dp[5] = 1;
    }
    for i in 6..=n {
        dp[i] = match (dp[i - 3], dp[i - 5]) {
            (-1, -1) => -1,
            (x1, -1) => x1 + 1,
            (-1, x2) => x2 + 1,
            (x1, x2) => x1.min(x2) + 1,
        }
    }

    write!(output, "{}", dp[n]).unwrap();
    print!("{output}");
}

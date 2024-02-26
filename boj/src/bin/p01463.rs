use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn dp(n: usize) -> usize {
    if n < 2 {
        0
    } else {
        (dp(n / 2) + (n % 2)).min(dp(n / 3) + (n % 3)) + 1
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let n: usize = input.trim().parse::<usize>().unwrap();
    write!(output, "{}", dp(n));
    print!("{output}");
}

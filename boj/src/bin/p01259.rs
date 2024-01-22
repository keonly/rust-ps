use std::fmt::Write;
use std::io::{stdin, Read};

fn is_palindrome(n: &str) -> bool {
    n.chars().rev().collect::<Vec<char>>() == n.chars().collect::<Vec<char>>()
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    while {
        stdin().read_line(&mut input).unwrap();
        input.trim() != "0"
    } {
        if is_palindrome(input.trim()) {
            writeln!(output, "yes").unwrap();
        } else {
            writeln!(output, "no").unwrap();
        }
        input.clear();
    }

    print!("{}", output);
}


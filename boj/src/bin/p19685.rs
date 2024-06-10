use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn is_palindrome(n: usize) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();

    let [s, e] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    for i in s..=e {
        if is_palindrome(i) {
            writeln!(output, "Palindrome!").unwrap();
        } else {
            writeln!(output, "{i}").unwrap();
        }
    }

    print!("{output}");
}

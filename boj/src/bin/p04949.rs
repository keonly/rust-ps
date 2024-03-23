use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => {
                stack.push(c);
            }
            ')' | '}' | ']' => {
                if let Some(prev) = stack.pop() {
                    if !(prev == '(' && c == ')'
                        || prev == '{' && c == '}'
                        || prev == '[' && c == ']')
                    {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let input = buffer.trim().lines();
    let mut output = String::new();

    for l in input {
        if l == "." {
            break;
        }
        if is_balanced(l) {
            writeln!(output, "yes").unwrap();
        } else {
            writeln!(output, "no").unwrap();
        }
    }

    print!("{output}");
}

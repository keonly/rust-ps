use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let input = buffer.trim().lines().skip(1);
    let mut output = String::new();

    let mut stack: Vec<usize> = vec![];
    for l in input {
        let num: usize = l.parse::<usize>().unwrap();
        match num {
            0 => {
                stack.pop();
            }
            _ => {
                stack.push(num);
            }
        }
    }

    write!(output, "{}", stack.iter().sum::<usize>()).unwrap();
    print!("{output}");
}

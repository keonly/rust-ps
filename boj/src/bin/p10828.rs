use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let mut stack: VecDeque<isize> = VecDeque::new();
    let lines = input.lines().skip(1);

    for l in lines {
        let mut comm = l.split_ascii_whitespace();
        let op = comm.next().unwrap();

        match op {
            "push" => {
                let num = comm.next().unwrap().parse::<isize>().unwrap();
                stack.push_back(num);
            }
            "pop" => {
                writeln!(output, "{}", stack.pop_back().unwrap_or(-1)).unwrap();
            }
            "size" => {
                writeln!(output, "{}", stack.len()).unwrap();
            }
            "empty" => {
                if stack.is_empty() {
                    writeln!(output, "1").unwrap();
                } else {
                    writeln!(output, "0").unwrap();
                }
            }
            "top" => {
                writeln!(output, "{}", stack.back().unwrap_or(&-1)).unwrap();
            }
            _ => (),
        };
    }

    print!("{output}");
}

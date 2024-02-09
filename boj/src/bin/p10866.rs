use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let mut deque: VecDeque<isize> = VecDeque::new();
    let lines = input.lines().skip(1);

    for l in lines {
        let mut comm = l.split_ascii_whitespace();
        let op = comm.next().unwrap();

        match op {
            "push_front" => {
                let num = comm.next().unwrap().parse::<isize>().unwrap();
                deque.push_front(num);
            }
            "push_back" => {
                let num = comm.next().unwrap().parse::<isize>().unwrap();
                deque.push_back(num);
            }
            "pop_front" => {
                writeln!(output, "{}", deque.pop_front().unwrap_or(-1)).unwrap();
            }
            "pop_back" => {
                writeln!(output, "{}", deque.pop_back().unwrap_or(-1)).unwrap();
            }
            "size" => {
                writeln!(output, "{}", deque.len()).unwrap();
            }
            "empty" => {
                if deque.is_empty() {
                    writeln!(output, "1").unwrap();
                } else {
                    writeln!(output, "0").unwrap();
                }
            }
            "front" => {
                writeln!(output, "{}", deque.front().unwrap_or(&-1)).unwrap();
            }
            "back" => {
                writeln!(output, "{}", deque.back().unwrap_or(&-1)).unwrap();
            }
            _ => (),
        };
    }

    print!("{output}");
}

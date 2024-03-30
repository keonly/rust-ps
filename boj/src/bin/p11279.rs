use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let n: usize = parse_next();
    let mut max_heap: BinaryHeap<usize> = BinaryHeap::new();

    for _ in 0..n {
        let x: usize = parse_next();
        match x {
            0 => {
                writeln!(
                    output,
                    "{}",
                    if let Some(val) = max_heap.pop() {
                        val
                    } else {
                        0
                    }
                )
                .unwrap();
            }
            _ => {
                max_heap.push(x);
            }
        }
    }
    print!("{output}");
}

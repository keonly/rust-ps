use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<isize>().unwrap();

    let n: isize = parse_next();
    let mut max_heap: BinaryHeap<isize> = BinaryHeap::new();
    let mut min_heap: BinaryHeap<isize> = BinaryHeap::new();

    for _ in 0..n {
        let x: isize = parse_next();
        match x.cmp(&0) {
            Ordering::Equal => match (max_heap.is_empty(), min_heap.is_empty()) {
                (false, false) => {
                    if max_heap.peek().unwrap() >= min_heap.peek().unwrap() {
                        writeln!(output, "{}", max_heap.pop().unwrap()).unwrap();
                    } else {
                        writeln!(output, "{}", -min_heap.pop().unwrap()).unwrap();
                    }
                }
                (false, true) => {
                    writeln!(output, "{}", max_heap.pop().unwrap()).unwrap();
                }
                (true, false) => {
                    writeln!(output, "{}", -min_heap.pop().unwrap()).unwrap();
                }
                (true, true) => {
                    writeln!(output, "0").unwrap();
                }
            },
            Ordering::Greater => {
                min_heap.push(-x);
            }
            Ordering::Less => {
                max_heap.push(x);
            }
        }
    }
    print!("{output}");
}

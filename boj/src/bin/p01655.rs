use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<isize>().unwrap();

    let n: isize = parse_next();
    let mut hi_heap: BinaryHeap<isize> = BinaryHeap::new();
    let mut lo_heap: BinaryHeap<isize> = BinaryHeap::new();

    for _ in 0..n {
        let x: isize = parse_next();
        if lo_heap.is_empty() || x <= *lo_heap.peek().unwrap() {
            lo_heap.push(x);
        } else {
            hi_heap.push(-x);
        }

        if hi_heap.len() > lo_heap.len() {
            lo_heap.push(-hi_heap.pop().unwrap());
        }
        if lo_heap.len() > hi_heap.len() + 1 {
            hi_heap.push(-lo_heap.pop().unwrap());
        }
        writeln!(output, "{}", lo_heap.peek().unwrap()).unwrap();
    }

    print!("{output}");
    let mut hi_heap: BinaryHeap<usize> = BinaryHeap::new();
}

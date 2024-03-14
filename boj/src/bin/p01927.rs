use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines().skip(1);
    let mut output = String::new();

    let mut heap: BinaryHeap<isize> = BinaryHeap::new();
    for l in input {
        let num: isize = l.parse::<isize>().unwrap();
        if num > 0 {
            heap.push(-num);
        } else {
            writeln!(output, "{}", -heap.pop().unwrap_or(0)).unwrap();
        }
    }

    print!("{output}");
}

use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let [start, target] = [(); 2].map(|_| parse_next());
    let mut count: isize = 1;
    let mut track: usize = target;
    while track != start {
        if track % 2 == 0 && track > 0 {
            track /= 2;
            count += 1;
        } else if track % 10 == 1 {
            track /= 10;
            count += 1;
        } else {
            count = -1;
            break;
        }
    }

    writeln!(output, "{count}").unwrap();
    print!("{output}");
}

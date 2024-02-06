use std::collections::HashSet;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let mut keys: HashSet<isize> = HashSet::new();
    input
        .lines()
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<isize>().unwrap())
        .for_each(|k| {
            keys.insert(k);
        });
    let queries = input
        .lines()
        .nth(3)
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<isize>().unwrap());

    queries.for_each(|q| {
        if keys.contains(&q) {
            writeln!(output, "1").unwrap();
        } else {
            writeln!(output, "0").unwrap();
        }
    });

    print!("{output}");
}

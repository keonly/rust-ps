use std::collections::HashMap;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let mut keys: HashMap<isize, usize> = HashMap::new();
    input
        .lines()
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<isize>().unwrap())
        .for_each(|k| {
            if let Some(e) = keys.get_mut(&k) {
                *e += 1;
            } else {
                keys.insert(k, 1);
            }
        });
    let queries = input
        .lines()
        .nth(3)
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<isize>().unwrap());

    queries.for_each(|q| {
        if let Some(count) = keys.get(&q) {
            write!(output, "{count} ").unwrap();
        } else {
            write!(output, "0 ").unwrap();
        }
    });

    print!("{output}");
}

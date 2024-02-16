use std::collections::HashSet;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.lines();
    let mut output = String::new();

    let mut nums = input.next().unwrap().split_ascii_whitespace();
    let [n, m] = [(); 2].map(|_| nums.next().unwrap().parse::<usize>().unwrap());

    let mut never_heard: HashSet<&str> = HashSet::new();
    let mut never_heard_nor_seen: Vec<String> = Vec::new();

    for _ in 0..n {
        let name = input.next().unwrap();
        never_heard.insert(name);
    }

    for _ in 0..m {
        let name = input.next().unwrap();
        if never_heard.contains(name) {
            never_heard_nor_seen.push(name.to_string());
        }
    }

    never_heard_nor_seen.sort_unstable();
    let count: usize = never_heard_nor_seen.len();
    writeln!(output, "{count}").unwrap();

    for name in never_heard_nor_seen.iter() {
        writeln!(output, "{name}").unwrap();
    }

    print!("{output}");
}

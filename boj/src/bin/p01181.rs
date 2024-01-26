use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let mut words: Vec<&str> = input
        .lines()
        .skip(1)
        .collect();
    words.sort_unstable_by(|l, r| l.len().cmp(&r.len()).then(l.cmp(r)));
    words.dedup();

    write!(output, "{}", words.join("\n")).unwrap();
    print!("{}", output);
}

use std::collections::HashMap;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let tc: usize = input.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..tc {
        let n: usize = input.next().unwrap().parse::<usize>().unwrap();
        let mut count: HashMap<String, usize> = HashMap::new();
        for _ in 0..n {
            let mut line = input.next().unwrap().split_ascii_whitespace();
            let [_name, kind] = [(); 2].map(|_| line.next().unwrap().to_string());
            count
                .entry(kind)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let possibilities = count.iter().fold(1, |acc, (_, count)| acc * (count + 1)) - 1;
        writeln!(output, "{possibilities}").unwrap();
    }

    print!("{output}");
}

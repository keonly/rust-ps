use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn dfs(start: usize, n: usize, m: usize, current: &mut Vec<usize>, output: &mut String) {
    if current.len() == m {
        writeln!(
            output,
            "{}",
            current
                .iter()
                .map(|&n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
        return;
    }

    for i in start..=n {
        current.push(i);
        dfs(i + 1, n, m, current, output);
        current.pop();
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let [n, m] = [(); 2].map(|_| parse_next());
    dfs(1, n, m, &mut vec![], &mut output);

    print!("{output}");
}

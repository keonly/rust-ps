use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let input = buffer.trim().lines().skip(1);
    let mut output = String::new();

    let queries: Vec<usize> = input.map(|c| c.parse::<usize>().unwrap()).collect();
    let n: usize = *queries.iter().max().unwrap();
    let mut padovan: Vec<usize> = vec![0; 5.max(n + 1)];
    padovan[0] = 1;
    padovan[1] = 1;
    padovan[2] = 1;
    padovan[3] = 2;
    padovan[4] = 2;

    for i in 5..n {
        padovan[i] = padovan[i - 1] + padovan[i - 5];
    }

    for q in queries {
        writeln!(output, "{}", padovan[q - 1]).unwrap();
    }
    print!("{output}");
}

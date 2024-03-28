use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn dynamic_programming(n: usize, scores: &[usize]) -> usize {
    if n == 1 {
        return scores[0];
    }

    let mut dp_single_step: Vec<usize> = vec![0; n];
    let mut dp_double_step: Vec<usize> = vec![0; n];
    dp_single_step[0] = scores[0];
    dp_single_step[1] = dp_single_step[0] + scores[1];
    dp_double_step[1] = scores[1];

    for i in 2..n {
        dp_single_step[i] = dp_double_step[i - 1] + scores[i];
        dp_double_step[i] = dp_single_step[i - 2].max(dp_double_step[i - 2]) + scores[i];
    }

    dp_single_step[n - 1].max(dp_double_step[n - 1])
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let scores: Vec<usize> = input.map(|c| c.parse::<usize>().unwrap()).collect();

    write!(output, "{}", dynamic_programming(n, &scores)).unwrap();
    print!("{output}");
}

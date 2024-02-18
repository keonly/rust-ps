use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn count_fibonacci(n: usize) -> (usize, usize) {
    if n == 0 {
        return (1, 0);
    } else if n == 1 {
        return (0, 1);
    }

    let mut prev_2_count: (usize, usize) = (1, 0);
    let mut prev_1_count: (usize, usize) = (0, 1);

    for _ in 2..=n {
        let curr = (
            prev_1_count.0 + prev_2_count.0,
            prev_1_count.1 + prev_2_count.1,
        );
        prev_2_count = prev_1_count;
        prev_1_count = curr;
    }

    prev_1_count
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let input = buffer.lines().skip(1).map(|c| c.parse::<usize>().unwrap());
    let mut output = String::new();

    for n in input {
        let counts = count_fibonacci(n);
        writeln!(output, "{} {}", counts.0, counts.1).unwrap();
    }

    print!("{output}");
}

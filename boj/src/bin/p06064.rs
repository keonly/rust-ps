use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn convert_year(m: usize, n: usize, x: usize, y: usize) -> Option<usize> {
    let (x, y) = (x % m, y % n);

    for i in 0..=n {
        let year = i * m + x;
        if year > 0 && year % n == y {
            return Some(year);
        }
    }

    None
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let t = parse_next();
    for _ in 0..t {
        let [m, n, x, y] = [(); 4].map(|_| parse_next());
        if let Some(year) = convert_year(m, n, x, y) {
            writeln!(output, "{year}").unwrap();
        } else {
            writeln!(output, "-1").unwrap();
        }
    }

    print!("{output}");
}

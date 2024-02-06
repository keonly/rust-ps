use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn check_vps(s: &str) -> bool {
    let mut stack: usize = 0;

    for c in s.chars() {
        if matches!(c, '(') {
            stack += 1;
        } else {
            if stack == 0 {
                return false;
            }
            stack -= 1;
        }
    }

    stack == 0
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    input.lines().skip(1).for_each(|l| {
        if check_vps(&l) {
            writeln!(output, "YES").unwrap();
        } else {
            writeln!(output, "NO").unwrap();
        }
    });
    print!("{output}");
}

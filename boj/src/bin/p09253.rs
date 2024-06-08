use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();

    let [s1, s2, s3] = [(); 3].map(|_| input.next().unwrap().to_string());

    if s1.contains(&s3) && s2.contains(&s3) {
        writeln!(output, "YES").unwrap();
    } else {
        writeln!(output, "NO").unwrap();
    }
    print!("{output}");
}

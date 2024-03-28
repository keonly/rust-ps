use std::collections::HashMap;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let mut params = input.next().unwrap().split_ascii_whitespace();
    let [given, required] = [(); 2].map(|_| params.next().unwrap().parse::<usize>().unwrap());

    let mut passwords: HashMap<String, String> = HashMap::new();

    for _ in 0..given {
        let mut line = input.next().unwrap().split_ascii_whitespace();
        let [website, password] = [(); 2].map(|_| line.next().unwrap().to_string());
        passwords.insert(website, password);
    }
    for _ in 0..required {
        let website = input.next().unwrap().to_string();
        writeln!(output, "{}", passwords.get(&website).unwrap()).unwrap();
    }
    print!("{output}");
}

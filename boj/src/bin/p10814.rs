use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn collect_into_vector(input: &str) -> Vec<(usize, String)> {
    input
        .lines()
        .skip(1)
        .filter_map(|l| {
            let parts: Vec<&str> = l.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let age: usize = parts[0].parse::<usize>().unwrap();
                let name: String = parts[1].to_string();
                Some((age, name))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let mut people: Vec<(usize, String)> = collect_into_vector(&input);
    people.sort_by_key(|k| k.0);

    people.iter().for_each(|(age, name)| {
        writeln!(output, "{age} {name}").unwrap();
    });
    print!("{output}");
}

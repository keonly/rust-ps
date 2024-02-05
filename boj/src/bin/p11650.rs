use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn collect_to_vector(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .skip(1)
        .filter_map(|l| {
            let parts: Vec<&str> = l.split_ascii_whitespace().collect();
            if parts.len() == 2 {
                let x = parts[0].parse::<isize>().unwrap();
                let y = parts[1].parse::<isize>().unwrap();
                Some((x, y))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let mut coords: Vec<(isize, isize)> = collect_to_vector(&input);
    coords.sort_by(|l, r| l.0.cmp(&r.0).then(l.1.cmp(&r.1)));

    coords.iter().for_each(|(x, y)| {
        writeln!(output, "{x} {y}").unwrap();
    });
    print!("{output}");
}

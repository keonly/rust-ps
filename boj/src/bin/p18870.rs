use std::cmp::Ordering;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn coord_compression(coords: &mut Vec<isize>) -> Vec<usize> {
    let mut sorted = coords.clone().to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    coords
        .to_vec()
        .iter()
        .map(|seek| {
            sorted
                .binary_search_by(|probe| match probe.cmp(seek) {
                    Ordering::Equal => Ordering::Greater,
                    ord => ord,
                })
                .unwrap_err()
        })
        .collect::<Vec<usize>>()
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.lines().skip(1);
    let mut output = String::new();

    let mut original: Vec<isize> = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<isize>().unwrap())
        .collect();

    let compressed: Vec<usize> = coord_compression(&mut original);
    for c in compressed.iter() {
        write!(output, "{} ", c).unwrap();
    }
    print!("{output}");
}

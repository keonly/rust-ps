use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn find_optimal_height(required: usize, trees: &[usize]) -> usize {
    let (mut lo, mut hi) = (0, *trees.iter().max().unwrap());
    let mut height = lo;

    while lo <= hi {
        let mid = (hi - lo) / 2 + lo;
        if trees
            .iter()
            .fold(0, |acc, val| acc + if val >= &mid { val - mid } else { 0 })
            < required
        {
            hi = mid - 1;
        } else {
            height = height.max(mid);
            lo = mid + 1;
        }
    }

    height
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let [_, required] = [(); 2].map(|_| parse_next());
    let trees: Vec<usize> = input.map(|c| c.parse::<usize>().unwrap()).collect();

    write!(output, "{}", find_optimal_height(required, &trees)).unwrap();
    print!("{output}");
}

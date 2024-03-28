use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn find_maximum_length(n: usize, cables: &[usize]) -> usize {
    let mut opt_len: usize = 0;
    let (mut lo, mut hi) = (1, *cables.iter().max().unwrap());

    while lo <= hi {
        let mid = (hi - lo) / 2 + lo;
        let count = cables.iter().fold(0, |acc, val| acc + val / mid);
        if count < n {
            hi = mid - 1;
        } else {
            opt_len = opt_len.max(mid);
            lo = mid + 1;
        }
    }

    opt_len
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let mut params = input.next().unwrap().split_ascii_whitespace();
    let [k, n] = [(); 2].map(|_| params.next().unwrap().parse::<usize>().unwrap());
    let cables: Vec<usize> = input.map(|c| c.parse::<usize>().unwrap()).collect();

    write!(output, "{}", find_maximum_length(n, &cables)).unwrap();
    print!("{output}");
}

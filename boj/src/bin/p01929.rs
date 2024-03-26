use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;

    (2..=n).for_each(|i| {
        if primes[i] {
            (i..=n).step_by(i).for_each(|j| {
                primes[j] = j == i;
            });
        }
    });

    primes
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();

    let [start, end] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let primes = sieve_of_eratosthenes(end);

    primes.iter().filter(|&&x| x >= start).for_each(|&x| {
        writeln!(output, "{x}").unwrap();
    });
    print!("{output}");
}

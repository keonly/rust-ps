use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(num, _)| num)
        .collect()
}

fn is_divisible_by(number: &str, divisor: usize) -> bool {
    let mut remainder = 0;
    for digit in number.chars() {
        remainder = remainder * 10 + (digit as usize - '0' as usize);
        remainder %= divisor;
    }

    remainder == 0
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();

    let p: String = input.next().unwrap().to_string();
    let k: usize = input.next().unwrap().parse().unwrap();

    let primes = sieve_of_eratosthenes(k - 1);
    let mut found = false;
    for &prime in &primes {
        if is_divisible_by(&p, prime) {
            writeln!(output, "BAD {}", prime).unwrap();
            found = true;
            break;
        }
    }
    if !found {
        writeln!(output, "GOOD").unwrap();
    }

    print!("{output}");
}

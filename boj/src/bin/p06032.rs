use std::cmp::Ordering;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

#[derive(Debug)]
struct Toy {
    index: usize,
    joy: usize,
    price: usize,
    happy_frugal: f64,
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let n = parse_next();
    let mut toys: Vec<_> = (1..=n)
        .map(|index| {
            let [joy, price] = [(); 2].map(|_| parse_next());
            let happy_frugal = joy as f64 / price as f64;
            Toy {
                index,
                joy,
                price,
                happy_frugal,
            }
        })
        .collect();

    toys.sort_unstable_by(|a, b| {
        b.happy_frugal
            .partial_cmp(&a.happy_frugal)
            .unwrap_or(Ordering::Equal)
    });
    let result = toys.iter().take(3);
    let total_price: usize = result.clone().map(|toy| toy.price).sum();

    writeln!(output, "{total_price}").unwrap();
    for toy in result {
        writeln!(output, "{}", toy.index).unwrap();
    }

    print!("{output}");
}

use std::collections::{HashMap, VecDeque};
use std::fmt::Write;
use std::io::{read_to_string, stdin};

struct Board {
    ladders: HashMap<usize, usize>,
    snakes: HashMap<usize, usize>,
}

impl Board {
    fn find_minimum_moves(&self) -> usize {
        let mut counts = [usize::MAX; 101];
        let mut visiting = VecDeque::new();
        visiting.push_back(1);
        counts[1] = 0;

        while let Some(curr) = visiting.pop_front() {
            for step in 1..=6 {
                let next = curr + step;
                let next = *self
                    .ladders
                    .get(&next)
                    .or_else(|| self.snakes.get(&next))
                    .unwrap_or(&next);

                if counts[next] > counts[curr] + 1 {
                    counts[next] = counts[curr] + 1;
                    visiting.push_back(next);
                }
                if next == 100 {
                    return counts[next];
                }
            }
        }

        counts[100]
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let [num_ladders, num_snakes] = [(); 2].map(|_| parse_next());
    let mut ladders: HashMap<usize, usize> = HashMap::new();
    let mut snakes: HashMap<usize, usize> = HashMap::new();
    for _ in 0..num_ladders {
        let [from, to] = [(); 2].map(|_| parse_next());
        ladders.insert(from, to);
    }
    for _ in 0..num_snakes {
        let [from, to] = [(); 2].map(|_| parse_next());
        snakes.insert(from, to);
    }
    let board = Board { ladders, snakes };

    write!(output, "{}", board.find_minimum_moves()).unwrap();
    print!("{output}");
}

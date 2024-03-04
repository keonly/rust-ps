use std::collections::{HashSet, VecDeque};
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn hide_and_seek(n: usize, k: usize) -> usize {
    let mut visiting: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();

    visiting.push_back((n, 0));

    while let Some((curr_pos, curr_step)) = visiting.pop_front() {
        if curr_pos == k {
            return curr_step;
        }
        if visited.contains(&curr_pos) || curr_pos > 200_000 {
            continue;
        }

        visited.insert(curr_pos);
        for next_pos in [curr_pos - 1, curr_pos + 1, curr_pos * 2] {
            visiting.push_back((next_pos, curr_step + 1))
        }
    }

    usize::MAX
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.split_ascii_whitespace();
    let mut output = String::new();

    let [n, k] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());

    write!(output, "{}", hide_and_seek(n, k)).unwrap();
    print!("{output}");
}

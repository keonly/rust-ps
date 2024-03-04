use std::collections::{HashSet, VecDeque};
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn hide_and_seek(n: isize, k: isize) -> isize {
    let mut visiting: VecDeque<(isize, isize)> = VecDeque::new();
    let mut visited: HashSet<isize> = HashSet::new();

    visiting.push_back((n, 0));

    while let Some((curr_pos, curr_step)) = visiting.pop_front() {
        if curr_pos == k {
            return curr_step;
        }

        visited.insert(curr_pos);
        for next_pos in [curr_pos - 1, curr_pos + 1, curr_pos * 2] {
            if visited.contains(&next_pos) || next_pos > 200_000 {
                continue;
            }
            visiting.push_back((next_pos, curr_step + 1))
        }
    }

    isize::MAX
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.split_ascii_whitespace();
    let mut output = String::new();

    let [n, k] = [(); 2].map(|_| input.next().unwrap().parse::<isize>().unwrap());

    write!(output, "{}", hide_and_seek(n, k)).unwrap();
    print!("{output}");
}

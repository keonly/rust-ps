use std::collections::{HashSet, VecDeque};
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn hide_and_seek(n: usize, k: usize) -> usize {
    let mut visiting: VecDeque<usize> = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut steps: usize = 0;

    visiting.push_back(n);

    'outer: loop {
        let temp = visiting.clone();
        let curr_step = temp.iter().to_owned();
        visiting.clear();

        for curr in curr_step {
            if *curr == k {
                break 'outer;
            }
            if visited.contains(curr) || *curr > 200_000 {
                continue;
            }
            visited.insert(*curr);
            for next in [curr - 1, curr + 1, curr * 2] {
                visiting.push_back(next);
            }
        }

        steps += 1;
    }

    steps
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.split_ascii_whitespace();
    let mut output = String::new();

    let [n, k] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());

    write!(output, "{}", hide_and_seek(n, k)).unwrap();
    print!("{output}");
}

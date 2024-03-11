use std::collections::{HashSet, VecDeque};
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse::<usize>().unwrap();
    let _ = input.next();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for l in input {
        let mut pairs = l.split_ascii_whitespace();
        let [v1, v2] = [(); 2].map(|_| pairs.next().unwrap().parse::<usize>().unwrap());
        graph[v1].push(v2);
        graph[v2].push(v1);
    }

    let mut visiting: VecDeque<usize> = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();
    visiting.push_back(1);

    while !visiting.is_empty() {
        let current = visiting.pop_front().unwrap();
        if !visited.contains(&current) {
            visited.insert(current);
            for v in graph[current].iter() {
                visiting.push_back(*v);
            }
        }
    }

    write!(output, "{}", visited.len() - 1).unwrap();
    print!("{output}");
}

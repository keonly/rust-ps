use std::collections::{HashSet, VecDeque};
use std::fmt::Write;
use std::io::{read_to_string, stdin};

struct Graph {
    vertices: usize,
    edges: usize,
    adjacency: Vec<Vec<usize>>,
}

impl Graph {
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visiting: VecDeque<usize> = VecDeque::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut result: Vec<usize> = vec![];
        visiting.push_back(start);

        while let Some(curr) = visiting.pop_front() {
            if !visited.contains(&curr) {
                result.push(curr);
            }
            visited.insert(curr);

            for next in self.adjacency[curr].iter().rev() {
                if !visited.contains(next) {
                    visiting.push_front(*next);
                }
            }
        }

        result
    }

    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visiting: VecDeque<usize> = VecDeque::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut result: Vec<usize> = vec![];
        visiting.push_back(start);

        while let Some(curr) = visiting.pop_front() {
            if !visited.contains(&curr) {
                result.push(curr);
            }
            visited.insert(curr);

            for next in self.adjacency[curr].iter() {
                if !visited.contains(next) {
                    visiting.push_back(*next);
                }
            }
        }

        result
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let [vertices, edges, start] = [(); 3].map(|_| parse_next());
    let mut adjacency: Vec<Vec<usize>> = vec![vec![]; vertices + 1];
    for _ in 0..edges {
        let [v1, v2] = [(); 2].map(|_| parse_next());
        adjacency[v1].push(v2);
        adjacency[v2].push(v1);
    }
    for vs in adjacency.iter_mut() {
        vs.sort_unstable();
    }

    let graph = Graph {
        vertices,
        edges,
        adjacency,
    };
    graph
        .dfs(start)
        .iter()
        .for_each(|x| write!(output, "{} ", x).unwrap());
    writeln!(output).unwrap();
    graph
        .bfs(start)
        .iter()
        .for_each(|x| write!(output, "{} ", x).unwrap());

    print!("{output}");
}

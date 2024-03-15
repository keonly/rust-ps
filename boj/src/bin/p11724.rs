use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

#[derive(Debug)]
struct UndirectedGraph {
    vertices: usize,
    edges: usize,
    graph: Vec<Vec<usize>>,
}

impl UndirectedGraph {
    fn bfs(&self, visited: &mut Vec<bool>, start_v: usize) {
        let mut visiting: VecDeque<usize> = VecDeque::new();
        visiting.push_back(start_v);
        visited[start_v] = true;

        while let Some(curr_v) = visiting.pop_front() {
            for next_v in self.graph[curr_v].iter() {
                if !visited[*next_v] {
                    visiting.push_back(*next_v);
                    visited[*next_v] = true;
                }
            }
        }
    }

    fn count_connected(&self) -> usize {
        let mut count = 0;
        let mut visited: Vec<bool> = vec![false; self.vertices];

        for i in 0..self.vertices {
            if !visited[i] {
                self.bfs(&mut visited, i);
                count += 1;
            }
        }

        count
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let mut dims = input.next().unwrap().split_ascii_whitespace();
    let [vertices, edges] = [(); 2].map(|_| dims.next().unwrap().parse::<usize>().unwrap());
    if edges == 0 {
        print!("{vertices}");
        return
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; vertices];

    for _ in 0..edges {
        let mut line = input.next().unwrap().split_ascii_whitespace();
        let [v1, v2] = [(); 2].map(|_| line.next().unwrap().parse::<usize>().unwrap());
        graph[v1 - 1].push(v2 - 1);
        graph[v2 - 1].push(v1 - 1);
    }

    let undirected_graph = UndirectedGraph {
        vertices,
        edges,
        graph,
    };

    write!(output, "{}", undirected_graph.count_connected()).unwrap();
    print!("{output}");
}

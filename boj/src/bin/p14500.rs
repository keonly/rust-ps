use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn find_max_sum(n: usize, m: usize, grid: &[Vec<usize>]) -> usize {
    let tetrominoes = vec![
        // I Shaped
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        // Square Shaped
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        // L Shaped
        vec![(0, 0), (1, 0), (2, 0), (2, 1)],
        vec![(0, 1), (1, 1), (2, 1), (2, 0)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2)],
        vec![(0, 0), (1, 0), (1, 1), (1, 2)],
        vec![(0, 0), (0, 1), (0, 2), (1, 0)],
        vec![(1, 0), (1, 1), (1, 2), (0, 2)],
        vec![(0, 1), (0, 0), (1, 0), (2, 0)],
        vec![(0, 0), (0, 1), (1, 1), (2, 1)],
        // Z Shaped
        vec![(0, 0), (0, 1), (1, 1), (1, 2)],
        vec![(1, 0), (1, 1), (0, 1), (0, 2)],
        vec![(0, 1), (1, 1), (1, 0), (2, 0)],
        vec![(0, 0), (1, 0), (1, 1), (2, 1)],
        // T Shaped
        vec![(0, 0), (0, 1), (0, 2), (1, 1)],
        vec![(1, 0), (1, 1), (1, 2), (0, 1)],
        vec![(0, 0), (1, 0), (2, 0), (1, 1)],
        vec![(0, 1), (1, 1), (2, 1), (1, 0)],
    ];

    let mut max_sum = 0;
    for i in 0..n {
        for j in 0..m {
            for tet in &tetrominoes {
                let sum = tet.iter().fold(0, |acc, &(dx, dy)| {
                    if i + dx < n && j + dy < m {
                        acc + grid[i + dx][j + dy]
                    } else {
                        0
                    }
                });
                if sum > max_sum {
                    max_sum = sum;
                }
            }
        }
    }

    max_sum
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let [n, m] = [(); 2].map(|_| parse_next());

    let grid: Vec<Vec<usize>> = (0..n)
        .map(|_| (0..m).map(|_| parse_next()).collect::<Vec<_>>())
        .collect();

    writeln!(output, "{}", find_max_sum(n, m, &grid)).unwrap();
    println!("{output}");
}

use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};
use std::option::Option;

#[derive(Debug)]
struct Graph {
    rows: usize,
    cols: usize,
    coords: Vec<Vec<u8>>,
}

impl Graph {
    fn in_range(&self, row: isize, col: isize) -> bool {
        0 <= row
            && row < self.rows as isize
            && 0 <= col
            && col < self.cols as isize
            && self.coords[row as usize][col as usize] != 0
    }

    fn bfs(&self, start: (usize, usize)) -> Vec<Vec<usize>> {
        let mut count: Vec<Vec<usize>> = vec![vec![usize::MAX; self.cols]; self.rows];
        let mut visiting: VecDeque<(usize, usize)> = VecDeque::new();
        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        visiting.push_back(start);

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.coords[i][j] == 0 {
                    count[i][j] = 0;
                }
            }
        }
        count[start.0][start.1] = 0;

        while let Some((curr_row, curr_col)) = visiting.pop_front() {
            for (d_row, d_col) in &directions {
                let (next_row, next_col) = (curr_row as isize + d_row, curr_col as isize + d_col);
                if self.in_range(next_row, next_col) {
                    let (next_row, next_col) = (next_row as usize, next_col as usize);
                    if count[next_row][next_col] == usize::MAX {
                        count[next_row][next_col] = count[curr_row][curr_col] + 1;
                        visiting.push_back((next_row, next_col));
                    }
                }
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
    let [rows, cols] = [(); 2].map(|_| dims.next().unwrap().parse::<usize>().unwrap());

    let mut coords: Vec<Vec<u8>> = vec![];
    let mut start: Option<(usize, usize)> = None;

    for (i, l) in input.enumerate() {
        let row = l.split_ascii_whitespace();
        let row_vec = row.map(|c| c.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        if let Some(j) = row_vec.iter().position(|&x| x == 2) {
            start = Some((i, j));
        }
        coords.push(row_vec);
    }

    let graph = Graph { rows, cols, coords };
    if let Some(start_coords) = start {
        let count = graph.bfs(start_coords);
        for row in count {
            for elem in row {
                if elem == usize::MAX {
                    write!(output, "-1 ").unwrap();
                } else {
                    write!(output, "{} ", elem).unwrap();
                }
            }
            writeln!(output).unwrap();
        }
        print!("{output}");
    } else {
        panic!("Starting index not found");
    }
}

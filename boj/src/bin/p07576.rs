use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

#[derive(Debug)]
struct Coords {
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<Option<bool>>>,
}

impl Coords {
    fn in_range(&self, row: isize, col: isize) -> bool {
        0 <= row && (row as usize) < self.rows && 0 <= col && (col as usize) < self.cols
    }

    fn find_ripe_coords(&self) -> Vec<(usize, usize)> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, &elem)| match elem {
                        Some(true) => Some((i, j)),
                        _ => None,
                    })
            })
            .collect()
    }

    fn contains_unripe(&self) -> bool {
        self.matrix
            .iter()
            .any(|row| row.iter().any(|&elem| elem == Some(false)))
    }

    fn bfs(&mut self) -> Option<usize> {
        let mut visiting: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let start_coords: Vec<(usize, usize)> = self.find_ripe_coords();
        let mut min_count = 0;
        start_coords.iter().for_each(|(row, col)| {
            visiting.push_back((*row, *col, 0));
        });

        while let Some((curr_row, curr_col, curr_step)) = visiting.pop_front() {
            min_count = min_count.max(curr_step);
            for (d_row, d_col) in &directions {
                let (next_row, next_col) = (curr_row as isize + d_row, curr_col as isize + d_col);
                if self.in_range(next_row, next_col) {
                    let (next_row, next_col) = (next_row as usize, next_col as usize);
                    if self.matrix[next_row][next_col] == Some(false) {
                        self.matrix[next_row][next_col] = Some(true);
                        visiting.push_back((next_row, next_col, curr_step + 1));
                    }
                }
            }
        }

        if !self.contains_unripe() {
            Some(min_count)
        } else {
            None
        }
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let mut dims = input.next().unwrap().split_ascii_whitespace();
    let [cols, rows] = [(); 2].map(|_| dims.next().unwrap().parse::<usize>().unwrap());
    let mut matrix: Vec<Vec<Option<bool>>> = vec![];

    for l in input {
        let row = l.split_ascii_whitespace();
        let row_vec = row
            .map(|c| match c {
                "0" => Some(false),
                "1" => Some(true),
                _ => None,
            })
            .collect::<Vec<Option<bool>>>();
        matrix.push(row_vec);
    }

    let mut coords = Coords { rows, cols, matrix };
    if let Some(min_count) = coords.bfs() {
        write!(output, "{min_count}").unwrap();
    } else {
        write!(output, "-1").unwrap();
    }
    print!("{output}");
}

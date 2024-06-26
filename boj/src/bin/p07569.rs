use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

#[derive(Debug)]
struct Coords {
    depth: usize,
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<Vec<Option<bool>>>>,
}

impl Coords {
    fn in_range(&self, depth: isize, row: isize, col: isize) -> bool {
        0 <= depth
            && (depth as usize) < self.depth
            && 0 <= row
            && (row as usize) < self.rows
            && 0 <= col
            && (col as usize) < self.cols
    }

    fn find_ripe_coords(&self) -> Vec<(usize, usize, usize)> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(k, layer)| {
                layer.iter().enumerate().flat_map(move |(i, row)| {
                    row.iter()
                        .enumerate()
                        .filter_map(move |(j, &elem)| match elem {
                            Some(true) => Some((k, i, j)),
                            _ => None,
                        })
                })
            })
            .collect()
    }

    fn contains_unripe(&self) -> bool {
        self.matrix.iter().any(|layer| {
            layer
                .iter()
                .any(|row| row.iter().any(|&elem| elem == Some(false)))
        })
    }

    fn bfs(&mut self) -> Option<usize> {
        let mut visiting: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
        let directions: [(isize, isize, isize); 6] = [
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ];
        let start_coords: Vec<(usize, usize, usize)> = self.find_ripe_coords();
        let mut min_count = 0;

        start_coords.iter().for_each(|(depth, row, col)| {
            visiting.push_back((*depth, *row, *col, 0));
        });

        while let Some((curr_depth, curr_row, curr_col, curr_step)) = visiting.pop_front() {
            min_count = min_count.max(curr_step);
            for (d_depth, d_row, d_col) in &directions {
                let (next_depth, next_row, next_col) = (
                    curr_depth as isize + d_depth,
                    curr_row as isize + d_row,
                    curr_col as isize + d_col,
                );
                if self.in_range(next_depth, next_row, next_col) {
                    let (next_depth, next_row, next_col) =
                        (next_depth as usize, next_row as usize, next_col as usize);
                    if self.matrix[next_depth][next_row][next_col] == Some(false) {
                        self.matrix[next_depth][next_row][next_col] = Some(true);
                        visiting.push_back((next_depth, next_row, next_col, curr_step + 1));
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
    let [cols, rows, depth] = [(); 3].map(|_| dims.next().unwrap().parse::<usize>().unwrap()); // Adjusted for 3D
    let mut matrix: Vec<Vec<Vec<Option<bool>>>> = vec![];

    for _ in 0..depth {
        let mut layer: Vec<Vec<Option<bool>>> = vec![];
        for _ in 0..rows {
            let row_vec = input
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|c| match c {
                    "0" => Some(false),
                    "1" => Some(true),
                    _ => None,
                })
                .collect::<Vec<Option<bool>>>();
            layer.push(row_vec);
        }
        matrix.push(layer);
    }

    let mut coords = Coords {
        depth,
        rows,
        cols,
        matrix,
    };
    if let Some(min_count) = coords.bfs() {
        write!(output, "{min_count}").unwrap();
    } else {
        write!(output, "-1").unwrap();
    }
    print!("{output}");
}

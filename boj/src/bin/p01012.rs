use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

#[derive(Debug)]
struct Coords {
    rows: usize,
    cols: usize,
    map: Vec<Vec<bool>>,
}

impl Coords {
    fn in_range(&self, row: isize, col: isize) -> bool {
        0 <= row && row < self.rows as isize && 0 <= col && col < self.cols as isize
    }

    fn bfs(&mut self, start: (usize, usize)) {
        let mut visiting: VecDeque<(usize, usize)> = VecDeque::new();
        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        visiting.push_back(start);

        while let Some((curr_row, curr_col)) = visiting.pop_front() {
            for (d_row, d_col) in &directions {
                let (next_row, next_col) = (curr_row as isize + d_row, curr_col as isize + d_col);
                if self.in_range(next_row, next_col) {
                    let (next_row, next_col) = (next_row as usize, next_col as usize);
                    if self.map[next_row][next_col] {
                        self.map[next_row][next_col] = false;
                        visiting.push_back((next_row, next_col));
                    }
                }
            }
        }
    }

    fn count_islands(&mut self) -> usize {
        let mut count = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.map[i][j] {
                    self.bfs((i, j));
                    count += 1;
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

    let test_cases: usize = input.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..test_cases {
        let mut dims = input.next().unwrap().split_ascii_whitespace();
        let [rows, cols, cabs] = [(); 3].map(|_| dims.next().unwrap().parse::<usize>().unwrap());
        let mut map: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

        for _ in 0..cabs {
            let mut locs = input.next().unwrap().split_ascii_whitespace();
            let [r, c] = [(); 2].map(|_| locs.next().unwrap().parse::<usize>().unwrap());
            map[r][c] = true;
        }
        let mut coords = Coords { rows, cols, map };
        writeln!(output, "{}", coords.count_islands()).unwrap();
    }

    print!("{output}");
}

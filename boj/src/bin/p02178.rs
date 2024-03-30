use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

struct Maze {
    rows: usize,
    cols: usize,
    map: Vec<Vec<bool>>,
}

impl Maze {
    fn in_range(&self, row: isize, col: isize) -> bool {
        0 <= row && row < self.rows as isize && 0 <= col && col < self.cols as isize
    }

    fn bfs(&self) -> usize {
        let mut visiting: VecDeque<(usize, usize)> = VecDeque::new();
        let mut track: Vec<Vec<usize>> = vec![vec![usize::MAX; self.cols]; self.rows];
        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        visiting.push_back((0, 0));
        track[0][0] = 1;

        while let Some((curr_row, curr_col)) = visiting.pop_front() {
            for (d_row, d_col) in &directions {
                let (next_row, next_col) = (curr_row as isize + d_row, curr_col as isize + d_col);
                if self.in_range(next_row, next_col) {
                    let (next_row, next_col) = (next_row as usize, next_col as usize);
                    if !self.map[next_row][next_col] {
                        continue;
                    }
                    if track[next_row][next_col] > track[curr_row][curr_col] + 1 {
                        track[next_row][next_col] = track[curr_row][curr_col] + 1;
                        visiting.push_back((next_row, next_col));
                    }
                }
            }
            if (curr_row, curr_col) == (self.rows - 1, self.cols - 1) {
                break;
            }
        }

        track[self.rows - 1][self.cols - 1]
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();

    let [rows, cols] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let map = (0..rows)
        .map(|_| {
            input
                .next()
                .unwrap()
                .chars()
                .map(|c| c == '1')
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    let maze = Maze { rows, cols, map };

    write!(output, "{}", maze.bfs()).unwrap();
    print!("{output}");
}

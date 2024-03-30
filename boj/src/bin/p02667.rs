use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

struct Map {
    size: usize,
    map: Vec<Vec<bool>>,
}

impl Map {
    fn in_range(&self, row: isize, col: isize) -> bool {
        0 <= row && row < self.size as isize && 0 <= col && col < self.size as isize
    }

    fn bfs(&mut self, start_row: usize, start_col: usize) -> usize {
        let mut visiting: VecDeque<(usize, usize)> = VecDeque::new();
        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut count = 1;
        visiting.push_back((start_row, start_col));
        self.map[start_row][start_col] = false;

        while let Some((curr_row, curr_col)) = visiting.pop_front() {
            for (d_row, d_col) in &directions {
                let (next_row, next_col) = (curr_row as isize + d_row, curr_col as isize + d_col);

                if self.in_range(next_row, next_col) {
                    let (next_row, next_col) = (next_row as usize, next_col as usize);

                    if self.map[next_row][next_col] {
                        self.map[next_row][next_col] = false;
                        visiting.push_back((next_row, next_col));
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn count_clusters(&mut self) -> Vec<usize> {
        let mut sizes = vec![];

        for i in 0..self.size {
            for j in 0..self.size {
                if self.map[i][j] {
                    sizes.push(self.bfs(i, j));
                }
            }
        }

        sizes
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let size: usize = input.next().unwrap().parse::<usize>().unwrap();
    let mut map: Vec<Vec<bool>> = (0..size)
        .map(|_| input.next().unwrap().chars().map(|c| c == '1').collect())
        .collect();
    let mut map = Map { size, map };
    let mut sizes = map.count_clusters();
    sizes.sort_unstable();

    writeln!(output, "{}", sizes.len()).unwrap();
    for x in sizes.iter() {
        writeln!(output, "{x}").unwrap();
    }
    print!("{output}");
}

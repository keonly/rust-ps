use std::collections::{HashSet, VecDeque};
use std::fmt::Write;
use std::io::{read_to_string, stdin};

struct Campus {
    rows: usize,
    cols: usize,
    map: Vec<Vec<char>>,
}

impl Campus {
    fn in_range(&self, row: isize, col: isize) -> bool {
        0 <= row && row < self.rows as isize && 0 <= col && col < self.cols as isize
    }

    fn count_reachable_people(&self, start_coord: (usize, usize)) -> usize {
        let mut visiting: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut count_people = 0;
        visiting.push_back(start_coord);
        visited.insert(start_coord);

        while let Some((curr_row, curr_col)) = visiting.pop_front() {
            for (d_row, d_col) in directions {
                let (next_row, next_col) = (curr_row as isize + d_row, curr_col as isize + d_col);
                if self.in_range(next_row, next_col) {
                    let (next_row, next_col) = (next_row as usize, next_col as usize);
                    if !visited.contains(&(next_row, next_col)) {
                        visited.insert((next_row, next_col));
                        let c = self.map[next_row][next_col];
                        match c {
                            'X' => {}
                            _ => {
                                visiting.push_back((next_row, next_col));
                                if c == 'P' {
                                    count_people += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        count_people
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();

    let [rows, cols] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut start_coord: Option<(usize, usize)> = None;
    let map: Vec<Vec<char>> = (0..rows)
        .map(|row_idx| {
            input
                .next()
                .unwrap()
                .chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    if c == 'I' && start_coord.is_none() {
                        start_coord = Some((row_idx, col_idx));
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();
    let campus = Campus { rows, cols, map };
    let count_people = campus.count_reachable_people(start_coord.unwrap());

    write!(
        output,
        "{}",
        if count_people > 0 {
            count_people.to_string()
        } else {
            "TT".to_string()
        }
    )
    .unwrap();
    print!("{output}");
}

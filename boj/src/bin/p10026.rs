use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

#[derive(Clone, Copy)]
enum Colour {
    Red,
    Green,
    Blue,
}

struct Picture {
    size: usize,
    pixels: Vec<Vec<Colour>>,
}

fn same_colour(col1: Colour, col2: Colour, colourblind: bool) -> bool {
    if colourblind {
        matches!(
            (col1, col2),
            (Colour::Red, Colour::Red)
                | (Colour::Green, Colour::Green)
                | (Colour::Blue, Colour::Blue)
                | (Colour::Green, Colour::Red)
                | (Colour::Red, Colour::Green)
        )
    } else {
        matches!(
            (col1, col2),
            (Colour::Red, Colour::Red)
                | (Colour::Green, Colour::Green)
                | (Colour::Blue, Colour::Blue)
        )
    }
}

impl Picture {
    fn in_range(&self, row: isize, col: isize) -> bool {
        0 <= row && (row as usize) < self.size && 0 <= col && (col as usize) < self.size
    }

    fn count_sections(&self, colourblind: bool) -> usize {
        let mut count = 0;
        let mut visited = vec![vec![false; self.size]; self.size];

        for i in 0..self.size {
            for j in 0..self.size {
                if !visited[i][j] {
                    self.bfs((i, j), colourblind, &mut visited);
                    count += 1;
                }
            }
        }

        count
    }

    fn bfs(&self, coords: (usize, usize), colourblind: bool, visited: &mut [Vec<bool>]) {
        let mut visiting = VecDeque::new();
        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        visiting.push_back(coords);
        visited[coords.0][coords.1] = true;

        while let Some((curr_row, curr_col)) = visiting.pop_front() {
            for (d_row, d_col) in &directions {
                let (next_row, next_col) = (curr_row as isize + d_row, curr_col as isize + d_col);
                if self.in_range(next_row, next_col) {
                    let (next_row, next_col) = (next_row as usize, next_col as usize);
                    if !visited[next_row][next_col]
                        && same_colour(
                            self.pixels[curr_row][curr_col],
                            self.pixels[next_row][next_col],
                            colourblind,
                        )
                    {
                        visited[next_row][next_col] = true;
                        visiting.push_back((next_row, next_col));
                    }
                }
            }
        }
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();

    let size = input.next().unwrap().parse::<usize>().unwrap();
    let pixels: Vec<Vec<_>> = input
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'R' => Colour::Red,
                    'G' => Colour::Green,
                    'B' => Colour::Blue,
                    _ => panic!("Unknown colour: {c}"),
                })
                .collect()
        })
        .collect();
    let picture = Picture { size, pixels };

    write!(
        output,
        "{} {}",
        picture.count_sections(false),
        picture.count_sections(true)
    )
    .unwrap();
    print!("{output}");
}

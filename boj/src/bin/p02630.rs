use std::fmt::Write;
use std::io::{read_to_string, stdin};

struct Paper {
    size: usize,
    matrix: Vec<Vec<bool>>,
}

impl Paper {
    fn is_perfect(&self, start_row: usize, start_col: usize, size: usize) -> Option<bool> {
        let first_elem = self.matrix[start_row][start_col];
        let all_same_colours = self.matrix[start_row..start_row + size]
            .iter()
            .flat_map(|row| &row[start_col..start_col + size])
            .all(|&element| element == first_elem);

        if !all_same_colours {
            None
        } else {
            Some(first_elem)
        }
    }

    fn count_perfects(&self, start_row: usize, start_col: usize, size: usize) -> (usize, usize) {
        if let Some(colour) = self.is_perfect(start_row, start_col, size) {
            if colour {
                (0, 1)
            } else {
                (1, 0)
            }
        } else {
            let half: usize = size / 2;
            let counts1 = self.count_perfects(start_row, start_col, half);
            let counts2 = self.count_perfects(start_row + half, start_col, half);
            let counts3 = self.count_perfects(start_row, start_col + half, half);
            let counts4 = self.count_perfects(start_row + half, start_col + half, half);

            (
                counts1.0 + counts2.0 + counts3.0 + counts4.0,
                counts1.1 + counts2.1 + counts3.1 + counts4.1,
            )
        }
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let size: usize = input.next().unwrap().parse::<usize>().unwrap();
    let mut matrix: Vec<Vec<bool>> = vec![];
    for l in input {
        let row = l.split_ascii_whitespace();
        let row_vec: Vec<bool> = row.map(|c| c != "0").collect::<Vec<bool>>();
        matrix.push(row_vec);
    }

    let paper = Paper { size, matrix };
    let counts = paper.count_perfects(0, 0, paper.size);

    writeln!(output, "{}", counts.0).unwrap();
    writeln!(output, "{}", counts.1).unwrap();
    print!("{output}");
}

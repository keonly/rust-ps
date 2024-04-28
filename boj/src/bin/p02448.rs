use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn draw_stars(canvas: &mut [Vec<char>], row: usize, col: usize, size: usize) {
    if size == 3 {
        canvas[row][col + 2] = '*';
        canvas[row + 1][col + 1] = '*';
        canvas[row + 1][col + 3] = '*';
        for i in 0..5 {
            canvas[row + 2][col + i] = '*';
        }
    } else {
        let new_size = size / 2;
        draw_stars(canvas, row, col + new_size, new_size);
        draw_stars(canvas, row + new_size, col, new_size);
        draw_stars(canvas, row + new_size, col + 2 * new_size, new_size);
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();
    let mut output = String::new();
    let mut canvas = vec![vec![' '; 2 * n - 1]; n];

    draw_stars(&mut canvas, 0, 0, n);

    for row in canvas {
        let line: String = row.iter().collect();
        writeln!(output, "{}", line).unwrap();
    }
    print!("{output}");
}

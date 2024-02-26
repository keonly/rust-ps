use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn find_visit(n: usize, r: usize, c: usize) -> usize {
    if n == 0 {
        0
    } else {
        let half = 2u32.pow(n as u32 - 1) as usize;
        match (r / half, c / half) {
            (0, 0) => find_visit(n - 1, r % half, c % half),
            (0, 1) => find_visit(n - 1, r % half, c % half) + 1 * (half * half),
            (1, 0) => find_visit(n - 1, r % half, c % half) + 2 * (half * half),
            (1, 1) => find_visit(n - 1, r % half, c % half) + 3 * (half * half),
            (x, y) => {
                panic!("Unexpected value: {x}, {y}");
            }
        }
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.split_ascii_whitespace();
    let mut output = String::new();

    let [n, r, c] = [(); 3].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    write!(output, "{}", find_visit(n, r, c)).unwrap();
    print!("{output}");
}

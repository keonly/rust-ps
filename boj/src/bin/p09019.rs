use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

mod calculator {
    pub fn d_op(reg: usize) -> usize {
        (reg * 2) % 10000
    }

    pub fn s_op(reg: usize) -> usize {
        if reg == 0 {
            9999
        } else {
            reg - 1
        }
    }

    pub fn l_op(reg: usize) -> usize {
        let left_digit = reg / 1000;
        let rest = reg % 1000;
        rest * 10 + left_digit
    }

    pub fn r_op(reg: usize) -> usize {
        let right_digit = reg % 10;
        let rest = reg / 10;
        right_digit * 1000 + rest
    }
}

fn bfs(reg: usize, target: usize) -> Vec<char> {
    if reg == target {
        return vec![];
    }

    let mut visiting: VecDeque<usize> = VecDeque::new();
    let mut visited: [bool; 10000] = [false; 10000];
    let mut ops_tracker = vec![vec![]; 10000];
    let operations: [(fn(usize) -> usize, char); 4] = [
        (calculator::d_op, 'D'),
        (calculator::s_op, 'S'),
        (calculator::l_op, 'L'),
        (calculator::r_op, 'R'),
    ];
    visiting.push_back(reg);
    visited[reg] = true;

    while let Some(curr_reg) = visiting.pop_front() {
        for &(op, op_char) in &operations {
            let next_reg = op(curr_reg);

            if !visited[next_reg] {
                visited[next_reg] = true;
                ops_tracker[next_reg] = [ops_tracker[curr_reg].as_slice(), &[op_char]].concat();

                if next_reg == target {
                    return ops_tracker[next_reg].clone();
                }

                visiting.push_back(next_reg);
            }
        }
    }

    panic!("Target not reached");
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let tc = parse_next();
    for _ in 0..tc {
        let [init, target] = [(); 2].map(|_| parse_next());
        let ops = bfs(init, target);
        writeln!(output, "{}", ops.iter().collect::<String>()).unwrap();
    }

    print!("{output}");
}

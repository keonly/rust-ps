use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn parse_expression(expr: &str) -> Vec<(char, isize)> {
    let mut tokens = Vec::new();
    let mut curr_num = String::new();

    for c in expr.chars() {
        match c {
            '0'..='9' => {
                curr_num.push(c);
            }
            '+' | '-' => {
                tokens.push(curr_num.clone());
                curr_num.clear();
                tokens.push(c.to_string());
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    tokens.push(curr_num);

    let first_num = tokens[0].parse::<isize>().unwrap();
    let mut structured_tokens = Vec::new();
    let mut it = tokens.iter().skip(1);
    structured_tokens.push(('+', first_num));

    while let Some(op) = it.next() {
        if let Some(num) = it.next() {
            structured_tokens.push((op.chars().next().unwrap(), num.parse::<isize>().unwrap()));
        }
    }

    structured_tokens
}

fn find_minimum(tokens: &[(char, isize)]) -> isize {
    let minus_index = tokens
        .iter()
        .position(|&(op, _)| op == '-')
        .unwrap_or(tokens.len());

    tokens
        .iter()
        .enumerate()
        .fold(0, |acc, (index, &(_, val))| {
            if index < minus_index {
                acc + val
            } else {
                acc - val
            }
        })
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let input = buffer.trim();
    let mut output = String::new();

    let tokens = parse_expression(input);
    write!(output, "{}", find_minimum(&tokens)).unwrap();
    print!("{output}");
}

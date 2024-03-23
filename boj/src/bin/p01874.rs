use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn stack_sequence(n: usize, nums: &Vec<usize>) -> Result<Vec<char>, ()> {
    let mut init: Vec<usize> = (1..=n).rev().collect();
    let mut temp: Vec<usize> = vec![];
    let mut result: Vec<char> = vec![];

    for x in nums {
        while !init.is_empty() && x >= init.last().unwrap() {
            temp.push(init.pop().unwrap());
            result.push('+');
        }
        if temp.is_empty() || temp.last().unwrap() != x {
            return Err(());
        }
        temp.pop().unwrap();
        result.push('-');
    }

    Ok(result)
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse::<usize>().unwrap();
    let nums: Vec<usize> = input.map(|c| c.parse::<usize>().unwrap()).collect();
    if let Ok(result) = stack_sequence(n, &nums) {
        result.iter().for_each(|x| {
            writeln!(output, "{x}").unwrap();
        })
    } else {
        writeln!(output, "NO").unwrap();
    }
    print!("{output}");
}

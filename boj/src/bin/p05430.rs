use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn simulate_operations(ops_str: &str, nums_count: usize, nums_str: &str) -> Result<String, ()> {
    let ops = ops_str.chars().collect::<Vec<char>>();
    let mut nums: VecDeque<usize>;
    if nums_count > 0 {
        nums = nums_str[1..nums_str.len() - 1]
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();
    } else {
        nums = VecDeque::new();
    }
    let mut forward: bool = true;

    for op in ops {
        match op {
            'R' => {
                forward = !forward;
            }
            'D' => {
                if nums.is_empty() {
                    return Err(());
                }
                if forward {
                    nums.pop_front();
                } else {
                    nums.pop_back();
                }
            }
            _ => panic!("Unexpected operation: {op}"),
        }
    }

    let iter: Box<dyn Iterator<Item = &usize>> = if forward {
        Box::new(nums.iter())
    } else {
        Box::new(nums.iter().rev())
    };

    let result: String = iter
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(",");
    Ok(result)
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let t: usize = input.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let ops_str = input.next().unwrap();
        let nums_count = input.next().unwrap().parse::<usize>().unwrap();
        let nums_str = input.next().unwrap();

        match simulate_operations(ops_str, nums_count, nums_str) {
            Ok(result) => {
                writeln!(output, "[{}]", result).unwrap();
            }
            Err(_) => {
                writeln!(output, "error").unwrap();
            }
        }
    }

    print!("{output}");
}

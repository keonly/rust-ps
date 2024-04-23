use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn dfs(nums: &[usize], start: usize, m: usize, current: &mut Vec<usize>, output: &mut String) {
    if current.len() == m {
        writeln!(
            output,
            "{}",
            current
                .iter()
                .map(|&n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
        return;
    }

    for i in start..nums.len() {
        current.push(nums[i]);
        dfs(nums, i, m, current, output);
        current.pop();
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace();
    let mut output = String::new();
    let mut parse_next = || input.next().unwrap().parse::<usize>().unwrap();

    let [n, m] = [(); 2].map(|_| parse_next());
    let mut nums = (0..n).map(|_| parse_next()).collect::<Vec<_>>();
    nums.sort_unstable();
    dfs(&nums, 0, m, &mut vec![], &mut output);

    print!("{output}");
}

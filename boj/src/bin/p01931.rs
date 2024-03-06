use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn greedy(times: &Vec<(usize, usize)>) -> usize {
    let mut count: usize = 0;
    let mut end_time: usize = 0;

    for t in times {
        if t.0 >= end_time {
            count += 1;
            end_time = t.1;
        }
    }

    count
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let input = buffer.trim().lines().skip(1);
    let mut output = String::new();

    let mut times: Vec<(usize, usize)> = input
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            parts[0]
                .parse::<usize>()
                .ok()
                .and_then(|fst| parts[1].parse::<usize>().ok().map(|snd| (fst, snd)))
        })
        .collect();
    times.sort_by(|fst, snd| fst.1.cmp(&snd.1).then(fst.0.cmp(&snd.0)));

    write!(output, "{}", greedy(&times)).unwrap();
    print!("{output}");
}

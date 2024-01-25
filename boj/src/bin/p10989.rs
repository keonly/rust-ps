use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();
    let time = input.trim().parse::<usize>().unwrap();
    input.clear();

    let mut counts: Vec<usize> = vec![0; 10000];
    for _ in 0..time {
        stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<usize>().unwrap();
        counts[n - 1] += 1;
        input.clear();
    }
    
    let mut writer = BufWriter::with_capacity(1 << 17, stdout());
    for i in 1..=10000 {
        for _ in 0..counts[i - 1] {
            writeln!(writer, "{i}").unwrap();
        }
    }
}

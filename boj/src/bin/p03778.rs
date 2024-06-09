use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn count_frequency(word: &str) -> [i32; 26] {
    let mut freq = [0; 26];
    for c in word.chars() {
        freq[c as usize - 'a' as usize] += 1;
    }

    freq
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let t: usize = input.next().unwrap().parse().unwrap();
    for i in 1..=t {
        let [s1, s2] = [(); 2].map(|_| input.next().unwrap());
        let freq1 = count_frequency(s1);
        let freq2 = count_frequency(s2);

        let total_changes: i32 = freq1
            .iter()
            .zip(freq2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        writeln!(output, "Case #{i}: {total_changes}").unwrap();
    }

    print!("{output}");
}

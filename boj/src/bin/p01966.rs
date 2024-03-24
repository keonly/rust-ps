use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

fn simulate_printer(queue: &mut VecDeque<(u8, bool)>) -> usize {
    let mut count = 0;
    while let Some((next_doc, desired)) = queue.pop_front() {
        let mut should_requeue = false;
        for &(doc, _) in queue.iter() {
            if doc > next_doc {
                should_requeue = true;
                break;
            }
        }

        if should_requeue {
            queue.push_back((next_doc, desired));
        } else {
            count += 1;
            if desired {
                break;
            }
        }
    }
    count
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let tc: usize = input.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..tc {
        let mut queries = input.next().unwrap().split_ascii_whitespace();
        let docs = input.next().unwrap().split_ascii_whitespace();

        let [_, query] = [(); 2].map(|_| queries.next().unwrap().parse::<usize>().unwrap());
        let mut queue: VecDeque<(u8, bool)> = docs
            .enumerate()
            .map(|(i, c)| {
                let x = c.parse::<u8>().unwrap();
                if i == query {
                    (x, true)
                } else {
                    (x, false)
                }
            })
            .collect();

        writeln!(output, "{}", simulate_printer(&mut queue)).unwrap();
    }
    print!("{output}");
}

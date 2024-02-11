use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    let mut writer = BufWriter::new(stdout());

    let mut set: u32 = 0;
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse::<usize>().unwrap();

    for _ in (0..n) {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let mut comm = input.split_ascii_whitespace();

        match comm.next().unwrap() {
            "add" => {
                set |= 1 << comm.next().unwrap().parse::<usize>().unwrap();
            }
            "remove" => {
                set &= !(1 << comm.next().unwrap().parse::<usize>().unwrap());
            }
            "check" => {
                if ((1 << comm.next().unwrap().parse::<usize>().unwrap()) & set) != 0 {
                    writeln!(writer, "1").unwrap();
                } else {
                    writeln!(writer, "0").unwrap();
                }
            }
            "toggle" => {
                set ^= 1 << comm.next().unwrap().parse::<usize>().unwrap();
            }
            "all" => {
                set = std::u32::MAX;
            }
            "empty" => {
                set = 0;
            }
            _ => (),
        };
    }
}

use std::cmp::Reverse;
use std::fmt::Write;
use std::io::{read_to_string, stdin};

struct World {
    inv: usize,
    map: Vec<Vec<u16>>,
}

impl World {
    fn calculate_completion_time(&self, height: u16) -> Option<usize> {
        let (surplus, deficit) = self.map.iter().flat_map(|row| row.iter()).fold(
            (0, 0),
            |(acc_surplus, acc_deficit), &curr_height| {
                let diff = curr_height as isize - height as isize;
                if diff > 0 {
                    (acc_surplus + diff as usize, acc_deficit)
                } else {
                    (acc_surplus, acc_deficit + (-diff as usize))
                }
            },
        );

        if surplus + self.inv < deficit {
            None
        } else {
            Some(2 * surplus + deficit)
        }
    }

    fn find_highest(&self) -> (usize, u16) {
        (0..=256)
            .filter_map(|height| {
                self.calculate_completion_time(height)
                    .map(|time| (time, height))
            })
            .min_by_key(|&(time, height)| (time, Reverse(height)))
            .unwrap()
    }
}

fn main() {
    let buffer = read_to_string(stdin()).unwrap();
    let mut input = buffer.trim().lines();
    let mut output = String::new();

    let mut dims = input.next().unwrap().split_ascii_whitespace();
    let [rows, _cols, inv] = [(); 3].map(|_| dims.next().unwrap().parse::<usize>().unwrap());
    let mut map: Vec<Vec<u16>> = vec![];

    for _ in 0..rows {
        let row = input.next().unwrap().split_ascii_whitespace();
        let row_vec = row.map(|c| c.parse::<u16>().unwrap()).collect::<Vec<u16>>();
        map.push(row_vec);
    }

    let world = World { inv, map };

    let (time, height) = world.find_highest();
    write!(output, "{} {}", time, height).unwrap();
    print!("{output}");
}

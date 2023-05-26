use std::io::{self, BufRead};

fn main() {
    let count: usize = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split(" ")
                .filter(|s| match s.len() {
                    2 | 4 | 3 | 7 => true,
                    _ => false,
                })
                .collect::<Vec<_>>()
                .len()
        })
        .sum();

    println!("{:?}", count);
}

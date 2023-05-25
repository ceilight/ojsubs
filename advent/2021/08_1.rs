use std::io::{self, BufRead};

fn main() {
    let count: usize = io::stdin()
        .lock()
        .lines()
        .map(|x| {
            let s = x.unwrap();
            let outstr = s.split(" | ").nth(1).unwrap();
            outstr
                .split(" ")
                .filter(|x| {
                    match x.len() {
                        2 | 4 | 3 | 7 => true,
                        _ => false,
                    }
                })
                .collect::<Vec<&str>>()
                .len()
        })
        .sum();

    println!("{:?}", count);
}
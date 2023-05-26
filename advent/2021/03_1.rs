use std::io::{self, BufRead};

fn main() {
    let mut ones = [0usize; 12];
    let mut total = 0usize;
    for line in io::stdin().lock().lines() {
        for (i, c) in line.unwrap().chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            }
        }
        total += 1;
    }
    let mut gamma = 0u32;
    for count in ones {
        gamma <<= 1;
        gamma |= (count > total / 2) as u32;
    }
    let epsilon = gamma ^ 0xFFF;
    println!("{:?}", gamma * epsilon);
}

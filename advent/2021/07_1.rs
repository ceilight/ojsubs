use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut a: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
    a.sort();
    let median = a[a.len() / 2];
    let cost: i32 = a.iter().map(|x| (x - median).abs()).sum();
    println!("{:?}", cost);
}

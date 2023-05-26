use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut crabs: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();
    let cost: i32 = (min_crab..=max_crab)
        .map(|pos| {
            crabs
                .iter()
                .map(|crab| (1i32..=((crab - pos).abs())).sum::<i32>())
                .sum()
        })
        .min()
        .unwrap();
    println!("{:?}", cost);
}

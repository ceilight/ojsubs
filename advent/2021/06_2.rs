use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut a: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
    let mut count = [0u64; 9]; // number of fish with timer i
    for f in a {
        count[f] += 1;
    }
    for _ in 0..256 {
        // newborn count = number of fish with timer 0 at last iteration
        count.rotate_left(1);
        // also those fish will get a timer of 6 after producing
        count[6] += count[8];
    }
    println!("{:?}", count.iter().sum::<u64>());
}

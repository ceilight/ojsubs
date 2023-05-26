// this is solved by simulation
// this can also be solved using 06_2.rs (just replace the iteration limit to 80)

use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut a: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
    for _ in 0..80 {
        let ln = a.len();
        for i in 0..ln {
            if a[i] == 0 {
                a[i] = 6;
                a.push(8);
            } else {
                a[i] -= 1;
            }
        }
    }
    println!("{:?}", a.len());
}

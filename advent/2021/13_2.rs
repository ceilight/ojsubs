use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines().map(|l| l.unwrap());
    let mut p = HashSet::new();
    loop {
        match lines.next() {
            None => (),
            Some(l) if l.is_empty() => break,
            Some(l) => {
                let mut sp = l.split(",");
                let x = sp.next().unwrap().parse::<usize>().unwrap();
                let y = sp.next().unwrap().parse::<usize>().unwrap();
                p.insert((x, y));
            }
        }
    }

    loop {
        match lines.next() {
            None => break,
            Some(l) => {
                let mut sp = l.split("=");
                let d = sp.next().unwrap().chars().last().unwrap();
                let n = sp.next().unwrap().parse::<usize>().unwrap();

                p = p
                    .into_iter()
                    .map(|(x, y)| match d {
                        'x' if x > n => (n * 2 - x, y),
                        'y' if y > n => (x, n * 2 - y),
                        _ => (x, y),
                    })
                    .collect();
            }
        };
    }

    let max_x = p.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = p.iter().map(|(_, y)| *y).max().unwrap();
    for j in 0..=max_y {
        for i in 0..=max_x {
            print!("{}", if p.contains(&(i, j)) { '#' } else { '.' });
        }
        println!("");
    }
}

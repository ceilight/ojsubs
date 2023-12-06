use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let wins: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = &line[line.find(':').unwrap() + 1..];
            let (win_cards, base_cards) = line.split_once('|').unwrap();
            let win_cards: HashSet<u32> = win_cards
                .split_whitespace()
                .flat_map(str::parse::<u32>)
                .collect();
            base_cards
                .split_whitespace()
                .filter(|&x| {
                    let x = x.parse::<u32>().unwrap();
                    win_cards.contains(&x)
                })
                .count()
        })
        .collect();
    println!("part 1: {:?}", part1(&wins));
    println!("part 2: {:?}", part2(&wins));
}

fn part1(wins: &[usize]) -> u64 {
    wins.iter()
        .flat_map(|c| (*c > 0).then_some(1u64 << (c - 1)))
        .sum()
}

fn part2(wins: &[usize]) -> usize {
    let n = wins.len();
    let mut counts = vec![1usize; n];
    for i in 0..n {
        for j in i + 1..(n.min(i + wins[i] + 1)) {
            counts[j] += counts[i];
        }
    }
    counts.into_iter().sum()
}

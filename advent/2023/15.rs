use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    removed: bool,
    focal: usize,
}

#[derive(Copy, Clone, Debug)]
enum Instruction<'a> {
    Set(&'a str, usize),
    Rem(&'a str),
}

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    let s = lines.next().unwrap();
    let s2 = s.clone();
    println!("Part 1: {:?}", part1(s));
    println!("Part 2: {:?}", part2(s2));
}

fn hash(key: &str) -> usize {
    key.chars()
        .fold(0usize, |h, c| (h + usize::from(c as u8)) * 17 % 256)
}

fn part1(data: String) -> usize {
    data.split(',').map(hash).sum()
}

fn part2(data: String) -> usize {
    let instructions = data.split(',').map(|p| {
        if let Some((label, focal)) = p.split_once('=') {
            let focal = focal.parse::<usize>().unwrap();
            Instruction::Set(label, focal)
        } else if let Some((label, _)) = p.split_once('-') {
            Instruction::Rem(label)
        } else {
            unreachable!();
        }
    });

    let mut buckets: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    for x in instructions {
        match x {
            Instruction::Set(label, focal) => {
                let bucket_idx = hash(label);
                if let Some(idx) = buckets[bucket_idx]
                    .iter()
                    .position(|&x| x.label == label && !x.removed)
                {
                    buckets[bucket_idx][idx].focal = focal;
                } else {
                    buckets[bucket_idx].push(Lens {
                        label,
                        removed: false,
                        focal,
                    });
                }
            }
            Instruction::Rem(label) => {
                let bucket_idx = hash(label);
                if let Some(idx) = buckets[bucket_idx]
                    .iter()
                    .position(|&x| x.label == label && !x.removed)
                {
                    buckets[bucket_idx][idx].removed = true
                }
            }
        };
    }

    buckets.iter_mut().for_each(|b| b.retain(|&x| !x.removed));

    buckets
        .iter()
        .enumerate()
        .map(|(h, b)| {
            b.iter().enumerate().fold(0usize, |tot, (i, lens)| {
                tot + (h + 1) * (i + 1) * lens.focal
            })
        })
        .sum()
}

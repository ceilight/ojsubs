use std::io::{self, BufRead};
use std::iter::zip;

#[derive(Debug, Clone, Copy)]
struct Transform {
    dst: usize,
    src: usize,
    len: usize,
}

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    let seeds = lines.next().unwrap();
    let seeds = seeds.split(':').nth(1).unwrap().trim();
    let seeds: Vec<_> = seeds
        .split_whitespace()
        .flat_map(str::parse::<usize>)
        .collect();
    lines.next();

    let mut layers: Vec<Vec<Transform>> = Vec::new();
    while let Some(layer) = parse_input_map(&mut lines) {
        layers.push(layer);
    }

    println!("Part 1: {:?}", part1(&seeds, &layers));
    println!("Part 2: {:?}", part2(&seeds, &layers));
}

fn parse_input_map<I>(mut lines: I) -> Option<Vec<Transform>>
where
    I: Iterator<Item = String>,
{
    if lines.next().is_none() {
        return None;
    }

    let mut transforms = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut nums = line.split_whitespace().flat_map(str::parse::<usize>);
        transforms.push(Transform {
            dst: nums.next().unwrap(),
            src: nums.next().unwrap(),
            len: nums.next().unwrap(),
        });
    }
    Some(transforms)
}

fn part1(seeds: &Vec<usize>, layers: &Vec<Vec<Transform>>) -> usize {
    seeds
        .iter()
        .map(|&x| {
            let mut res = x;
            for layer in layers.iter() {
                if let Some(t) = layer.iter().find(|&t| res >= t.src && res < t.src + t.len) {
                    res = res - t.src + t.dst;
                }
            }
            res
        })
        .min()
        .unwrap()
}

fn part2(seeds: &Vec<usize>, layers: &Vec<Vec<Transform>>) -> usize {
    // i shouldn't have tried the brute force approach :skull:
    // (top 10 bits of trolling that went too far)

    // convert seed list elements into pairs of (start, len) of seed range
    // the seed ranges are replaced for every layer of mapping
    let mut seed_ranges = zip(seeds.iter().step_by(2), seeds.iter().skip(1).step_by(2))
        .map(|(&a, &b)| (a, b))
        .collect();

    for layer in layers.iter() {
        let mut next_ranges = Vec::new();

        // for every seed ranges, find sections that can be mapped
        // using the available maps in the current layers
        // the remaining sections are directly inserted into the next seed ranges
        for (start, len) in seed_ranges {
            let mut start = start;
            let mut len = len;

            while len > 0 {
                let mut dist: usize = len;
                let mut found = false;

                for r in layer.iter() {
                    if start >= r.src && start < r.src + r.len {
                        let offset = start - r.src;
                        let frag_len = len.min(r.len - offset);
                        next_ranges.push((offset + r.dst, frag_len));
                        len -= frag_len;
                        start += frag_len;
                        found = true;
                        break;
                    } else if start < r.src {
                        dist = dist.min(r.src - start);
                    }
                }

                if !found {
                    dist = dist.min(len);
                    next_ranges.push((start, dist));
                    len -= dist;
                    start += dist;
                }
            }
        }
        seed_ranges = next_ranges;
    }
    // the smallest leftmost location index of all resulting ranges
    seed_ranges.iter().map(|(st, _)| *st).min().unwrap()
}

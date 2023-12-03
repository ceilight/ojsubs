use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct R((usize, usize, usize));

fn main() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (_, l) = l.split_once(": ").unwrap();
            l.split("; ")
                .map(|t| {
                    let mut tmp = (0usize, 0usize, 0usize);
                    let records = t.split(", ");
                    for r in records {
                        let (count, kind) = r.split_once(' ').unwrap();
                        let count = count.parse::<usize>().unwrap();
                        match kind {
                            "red" => tmp.0 += count,
                            "green" => tmp.1 += count,
                            "blue" => tmp.2 += count,
                            _ => (),
                        }
                    }
                    R(tmp)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    println!("part 1: {:?}", part1(&data));
    println!("part 2: {:?}", part2(&data));
}

fn part1(data: &[Vec<R>]) -> usize {
    data.iter()
        .enumerate()
        .filter(|&(_, g)| g.iter().all(|&R(x)| x.0 <= 12 && x.1 <= 13 && x.2 <= 14))
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(data: &[Vec<R>]) -> usize {
    data.iter()
        .map(|g| {
            let mx = g.iter().fold((0usize, 0usize, 0usize), |v, R(x)| {
                (v.0.max(x.0), v.1.max(x.1), v.2.max(x.2))
            });
            mx.0 * mx.1 * mx.2
        })
        .sum()
}

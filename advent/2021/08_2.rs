use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let tot: u32 = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut sp = line.split(" | ");

            let pat = sp.next().unwrap();
            let mut pat = pat
                .split(" ")
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();

            let mut seg = HashMap::<u8, _>::new();
            seg.insert(
                1,
                pat.swap_remove(pat.iter().position(|h| h.len() == 2).unwrap()),
            );
            seg.insert(
                4,
                pat.swap_remove(pat.iter().position(|h| h.len() == 4).unwrap()),
            );
            seg.insert(
                7,
                pat.swap_remove(pat.iter().position(|h| h.len() == 3).unwrap()),
            );
            seg.insert(
                8,
                pat.swap_remove(pat.iter().position(|h| h.len() == 7).unwrap()),
            );

            let v: Vec<_> = pat
                .iter()
                .enumerate()
                .filter(|(_, h)| h.len() == 6)
                .map(|(i, h)| {
                    let n = if seg[&1].intersection(h).count() == 1 {
                        6
                    } else if seg[&4].intersection(h).count() == 3 {
                        0
                    } else {
                        9
                    };
                    (i, n)
                })
                .collect();
            for (i, n) in v.into_iter().rev() {
                seg.insert(n, pat.swap_remove(i));
            }

            let v: Vec<_> = pat
                .iter()
                .enumerate()
                .filter(|(_, h)| h.len() == 5)
                .map(|(i, h)| {
                    let n = if seg[&1].intersection(h).count() == 2 {
                        3
                    } else if seg[&6].intersection(h).count() == 5 {
                        5
                    } else {
                        2
                    };
                    (i, n)
                })
                .collect();
            for (i, n) in v.into_iter().rev() {
                seg.insert(n, pat.swap_remove(i));
            }

            let mut num = 0;
            for r in sp
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.chars().collect::<HashSet<char>>())
            {
                let (k, _) = seg.iter().find(|(_, v)| &r == *v).unwrap();
                num = num * 10 + *k as u32;
            }
            num
        })
        .sum();

    println!("{}", tot);
}

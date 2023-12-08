use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    let instructions = lines.next().unwrap();
    let mut instructions = instructions.bytes().cycle();

    lines.next();
    let lines: Vec<_> = lines.collect();
    let network: HashMap<_, _> = lines
        .iter()
        .map(|line| {
            let mut s = line.split('=');
            let k = s.next().unwrap();
            let v = s.next().unwrap();
            let mut v = v[2..v.len() - 1].split(',');
            (
                k.trim(),
                (v.next().unwrap().trim(), v.next().unwrap().trim()),
            )
        })
        .collect();

    let mut cur = "AAA";
    let mut steps = 0;
    while cur != "ZZZ" {
        cur = match instructions.next() {
            Some(b'L') => network.get(&cur).unwrap().0,
            Some(b'R') => network.get(&cur).unwrap().1,
            _ => continue,
        };
        steps += 1;
    }
    println!("{:?}", steps);
}

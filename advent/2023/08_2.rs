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

    let all_steps = network.keys().filter(|k| k.ends_with('A')).map(|&k| {
        let mut cur = k;
        let mut steps = 0u64;
        loop {
            cur = match instructions.next() {
                Some(b'L') => network.get(&cur).unwrap().0,
                Some(b'R') => network.get(&cur).unwrap().1,
                _ => continue,
            };
            steps += 1;
            if cur.ends_with('Z') {
                return steps;
            }
        }
    });
    println!("{:?}", lcm(all_steps));
}

fn lcm<I>(mut nums: I) -> u64
where
    I: Iterator<Item = u64>,
{
    let mut lcm = if let Some(num) = nums.next() {
        num
    } else {
        return 0;
    };
    for x in nums {
        lcm = lcm / gcd(lcm, x) * x;
    }
    lcm
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

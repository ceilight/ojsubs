use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    let instructions = lines.next().unwrap();
    let instructions: Vec<_> = instructions.bytes().collect();

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

    println!("Part 1: {:?}", part1(&instructions, &network));
    println!("Part 2: {:?}", part2(&instructions, &network));
}

fn part1(instructions: &[u8], network: &HashMap<&str, (&str, &str)>) -> usize {
    let mut instructions = instructions.iter().cycle();
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
    steps
}

fn part2(instructions: &[u8], network: &HashMap<&str, (&str, &str)>) -> usize {
    let mut instructions = instructions.iter().cycle();
    let all_steps = network.keys().filter(|k| k.ends_with('A')).map(|&k| {
        let mut cur = k;
        let mut steps = 0;
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
    lcm(all_steps)
}

fn lcm<I>(mut nums: I) -> usize
where
    I: Iterator<Item = usize>,
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

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
    is_on: bool,
}

impl Cuboid {
    fn parse(line: &str) -> Self {
        let (state, ranges) = line.split_once(' ').unwrap();
        let mut ranges = ranges.split(',');

        fn get_range(range: &str) -> (i32, i32) {
            let i = range.find('=').unwrap();
            let mut range = range[i + 1..].split("..");
            let lo = range.next().unwrap().parse().unwrap();
            let hi = range.next().unwrap().parse().unwrap();
            assert!(lo <= hi);
            (lo, hi)
        }

        Self {
            x: get_range(ranges.next().unwrap()),
            y: get_range(ranges.next().unwrap()),
            z: get_range(ranges.next().unwrap()),
            is_on: state == "on",
        }
    }

    fn intersection(&self, other: &Cuboid) -> Option<Self> {
        let x = (self.x.0.max(other.x.0), self.x.1.min(other.x.1));
        let y = (self.y.0.max(other.y.0), self.y.1.min(other.y.1));
        let z = (self.z.0.max(other.z.0), self.z.1.min(other.z.1));

        if x.0 > x.1 || y.0 > y.1 || z.0 > z.1 {
            return None;
        }
        let is_on = if self.is_on && other.is_on {
            false
        } else if !self.is_on && !other.is_on {
            true
        } else {
            other.is_on
        };
        Some(Self { x, y, z, is_on })
    }

    fn volume(&self) -> i64 {
        let dx = (self.x.1 - self.x.0 + 1) as i64;
        let dy = (self.y.1 - self.y.0 + 1) as i64;
        let dz = (self.z.1 - self.z.0 + 1) as i64;
        let v = dx * dy * dz;
        if self.is_on {
            v
        } else {
            -v
        }
    }
}

// Original solution for part 1
#[allow(dead_code)]
fn part_1_orig(cuboids: &[Cuboid]) -> usize {
    let mut cube_state: HashMap<(i32, i32, i32), bool> = HashMap::new();
    for cuboid in cuboids.iter() {
        for x in cuboid.x.0.max(-50)..=cuboid.x.1.min(50) {
            for y in cuboid.y.0.max(-50)..=cuboid.y.1.min(50) {
                for z in cuboid.z.0.max(-50)..=cuboid.z.1.min(50) {
                    *cube_state.entry((x, y, z)).or_default() = cuboid.is_on;
                }
            }
        }
    }
    cube_state.iter().filter(|(_, v)| **v).count()
}

// Reuses part 2 solution to solve part 1
fn part_1(cuboids: &[Cuboid]) -> i64 {
    fn adjusted_cuboid(c: &Cuboid) -> Option<Cuboid> {
        let x = (c.x.0.max(-50), c.x.1.min(50));
        let y = (c.y.0.max(-50), c.y.1.min(50));
        let z = (c.z.0.max(-50), c.z.1.min(50));
        if x.0 > x.1 || y.0 > y.1 || z.0 > z.1 {
            return None;
        }
        Some(Cuboid {
            x,
            y,
            z,
            is_on: c.is_on,
        })
    }
    let mut init_cuboids = Vec::new();
    for cuboid in cuboids.iter() {
        if let Some(ac) = adjusted_cuboid(cuboid) {
            init_cuboids.push(ac);
        }
    }
    part_2(&init_cuboids)
}

fn part_2(cuboids: &[Cuboid]) -> i64 {
    let mut known: Vec<Cuboid> = Vec::new();
    for cuboid in cuboids.iter() {
        let mut pending = Vec::new();
        for existing in known.iter() {
            if let Some(i) = existing.intersection(cuboid) {
                pending.push(i);
            }
        }
        if cuboid.is_on {
            pending.push(*cuboid);
        }
        known.extend(pending);
    }
    known.iter().map(|x| x.volume()).sum()
}

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    let mut cuboids = Vec::new();
    while let Some(l) = lines.next() {
        cuboids.push(Cuboid::parse(&l));
    }

    let ans = part_1(&cuboids);
    println!("Part 1: {}", ans);
    let ans = part_2(&cuboids);
    println!("Part 2: {}", ans);
}

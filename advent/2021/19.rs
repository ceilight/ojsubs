use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type Pt = (i32, i32, i32);

const ROTATIONS: [fn(Pt) -> Pt; 24] = [
    |p| (p.0, p.1, p.2),
    |p| (p.0, -p.2, p.1),
    |p| (p.0, -p.1, -p.2),
    |p| (p.0, p.2, -p.1),
    |p| (-p.0, p.2, p.1),
    |p| (-p.0, -p.1, p.2),
    |p| (-p.0, -p.2, -p.1),
    |p| (-p.0, p.1, -p.2),
    |p| (p.1, p.2, p.0),
    |p| (p.1, -p.0, p.2),
    |p| (p.1, -p.2, -p.0),
    |p| (p.1, p.0, -p.2),
    |p| (-p.1, p.0, p.2),
    |p| (-p.1, -p.2, p.0),
    |p| (-p.1, -p.0, -p.2),
    |p| (-p.1, p.2, -p.0),
    |p| (p.2, p.0, p.1),
    |p| (p.2, -p.1, p.0),
    |p| (p.2, -p.0, -p.1),
    |p| (p.2, p.1, -p.0),
    |p| (-p.2, p.1, p.0),
    |p| (-p.2, -p.0, p.1),
    |p| (-p.2, -p.1, -p.0),
    |p| (-p.2, p.0, -p.1),
];

fn parse_report<I>(mut lines: I) -> Option<Vec<Pt>>
where
    I: Iterator<Item = String>,
{
    if lines.next().is_none() {
        return None;
    }
    let mut coords = vec![];
    for l in lines {
        if l.is_empty() {
            break;
        }
        let mut s = l.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let z = s.next().unwrap().parse().unwrap();
        coords.push((x, y, z));
    }
    Some(coords)
}

fn diff((l, r): &(Pt, Pt)) -> Pt {
    (r.0 - l.0, r.1 - l.1, r.2 - l.2)
}

fn fix_report_with(base_report: &[Pt], raw_report: &[Pt]) -> Option<(Vec<Pt>, Pt)> {
    for rotate in ROTATIONS {
        let mut overlap = HashMap::new();

        for i in 0..base_report.len() {
            'base: for j in i + 1..base_report.len() {
                let base_pair = (base_report[i], base_report[j]);
                let base_diff = diff(&base_pair);

                for u in 0..raw_report.len() {
                    for v in 0..raw_report.len() {
                        if u == v {
                            continue;
                        }
                        let raw_pair = (rotate(raw_report[u]), rotate(raw_report[v]));

                        if base_diff == diff(&raw_pair) {
                            let offset = diff(&(raw_pair.0, base_pair.0));

                            let m = overlap.entry(offset).or_insert_with(Vec::new);
                            m.push((i, u));
                            m.push((j, v));

                            if m.len() >= 12 {
                                let fixed_report = raw_report
                                    .iter()
                                    .map(|p| {
                                        let p = rotate(*p);
                                        (p.0 + offset.0, p.1 + offset.1, p.2 + offset.2)
                                    })
                                    .collect();
                                return Some((fixed_report, offset));
                            }
                            continue 'base;
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let mut fixed = HashMap::new();
    fixed.insert(0, parse_report(&mut lines).unwrap());

    let mut raw = vec![];
    while let Some(r) = parse_report(&mut lines) {
        raw.push(r);
    }
    let raw = raw;

    let num_scanners = raw.len() + 1;
    let mut scanners = vec![(0, 0, 0)];

    let mut done = HashSet::new();
    let mut fix_one = move |fixed: &HashMap<usize, Vec<Pt>>, raw: &[Vec<Pt>]| {
        for (i, f) in fixed.iter() {
            let i = *i;
            for (j, r) in raw.iter().enumerate() {
                let j = j + 1;
                if i == j || done.contains(&(i, j)) || done.contains(&(j, i)) {
                    continue;
                }
                done.insert((i, j));
                if let Some((fixed_u, scanner_pos)) = fix_report_with(f, r) {
                    return (j, fixed_u, scanner_pos);
                }
            }
        }
        unreachable!("Adjustable scanner couldn't be found");
    };

    while scanners.len() < num_scanners {
        let (num, report, scanner) = fix_one(&fixed, &raw);
        if fixed.insert(num, report).is_none() {
            scanners.push(scanner);
        }
    }
    
    let beacons: HashSet<_> = fixed.values().flat_map(|b| b.iter()).collect(); 
    println!("Part 1: {}", beacons.len());

    let max: i32 = scanners
        .iter()
        .map(|a| {
            scanners
                .iter()
                .filter(|b| *b != a)
                .map(|b| (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("Part 2: {}", max);
}
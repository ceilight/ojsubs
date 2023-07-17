// significantly faster solution, reducing runtime from ~6500 ms to ~15 ms

use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
// use std::fs;

type Pt = (i32, i32, i32);

// (Short attention span be damned this fella could work a rotation list on paper)
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

// Parses a scanner's report at a time
fn parse_report<I>(mut lines: I) -> Option<Vec<Pt>>
where
    I: Iterator<Item = String>,
{
    if lines.next().is_none() {
        return None;
    }
    let mut report = vec![];
    for l in lines {
        if l.is_empty() {
            break;
        }
        let mut s = l.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let z = s.next().unwrap().parse().unwrap();
        report.push((x, y, z));
    }
    Some(report)
}

// Calculates squared euclidean distance
// The value is used to arrange pairs of points in a report by their distance
// regardless of the distance's orientation (u->v or v->u)
fn dist(u: &Pt, v: &Pt) -> i32 {
    let (x, y, z) = (u.0 - v.0, u.1 - v.1, u.2 - v.2);
    x * x + y * y + z * z
}

// Maps euclidean distance to a list of corresponding pairs of point indices
type PM = HashMap<i32, Vec<(usize, usize)>>;

// Returns a hashmap containing all pairs of point indices in a report
// mapped by their squared euclidean distance
fn pair_dists(report: &[Pt]) -> PM {
    let mut dists = HashMap::new();
    let len = report.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            let k = dist(&report[i], &report[j]);
            dists.entry(k).or_insert_with(Vec::new).push((i, j));
        }
    }
    dists
}

// Calculates the number of possible overlapped pairs whose distance is in both maps
fn overlapped_pair_count(a: &PM, b: &PM) -> usize {
    a.iter()
        .filter(|(k, _)| b.contains_key(k))
        .map(|(k, v)| {
            let other = b.get(k).unwrap();
            v.len().min(other.len())
        })
        .sum()
}

// Maps a point index in an unfixed report to the list of potential matching
// point indices in a fixed report
type MC = HashMap<usize, HashSet<usize>>;

// Returns lists of point indices in a fixed report that are potentially matched
// with a point in the base report
fn match_candidates(unfixed_pm: &PM, base_pm: &PM) -> MC {
    let mut cands = HashMap::new();

    // Select all pairs with the same distance in both reports
    for (dist, unfixed_pairs) in unfixed_pm.iter() {
        if let Some(base_pairs) = base_pm.get(dist) {
            // Flatten list of indice pairs into unique set of indices
            let unfixed_pts = unfixed_pairs.iter().fold(HashSet::new(), |mut s, pair| {
                s.insert(pair.0);
                s.insert(pair.1);
                s
            });
            let base_pts = base_pairs.iter().fold(HashSet::new(), |mut s, pair| {
                s.insert(pair.0);
                s.insert(pair.1);
                s
            });
            // Map each element in unfixed_pts to all elements in fixed_pts
            for pt in unfixed_pts.iter() {
                cands
                    .entry(*pt)
                    .or_insert_with(HashSet::new)
                    .extend(&base_pts);
            }
        }
    }
    cands
}

// Adjusts unfixed report based on the base report that is already fixed
// The report is considered fixed if it contains at least 12 adjusted coordinates found in
// the base one.
fn fix(unfixed_report: &[Pt], base_report: &[Pt], cands: &MC) -> Option<(Vec<Pt>, Pt)> {
    // Any point `u` in the unfixed report whose index `i` contained in `cands` is guaranteed
    // to be matched with a point `v` in the base report whose index `j` is in a set mapped
    // to `i` given a suitable rotation.
    //
    // The approach:
    // * Get any point `u` in unfixed_report that contains potential matches in base report,
    // * For every potential match `v` in the base report, and every rotation:
    //   - Adjust `u` based on the rotation
    //   - Calculate the offset (vector difference of `u` and `v`)
    //   - Set up a proposed report with points in the unfixed report added with the offset
    //   - If the proposed report has at least 12 points also contained in the base report,
    //     it is fixed report and the offset is the scanner's coordinate
    let mut cands = cands.iter();
    let (i, js) = cands.next().unwrap();
    for j in js.iter() {
        let (i, j) = (*i, *j);
        let v = base_report[j];
        for rotate in ROTATIONS {
            let u = rotate(unfixed_report[i]);
            let offset = (v.0 - u.0, v.1 - u.1, v.2 - u.2);

            let proposed_report: Vec<Pt> = unfixed_report
                .iter()
                .map(|p| {
                    let p = rotate(*p);
                    (p.0 + offset.0, p.1 + offset.1, p.2 + offset.2)
                })
                .collect();

            let match_count = proposed_report
                .iter()
                .filter(|x| base_report.contains(x))
                .count();
            if match_count >= 12 {
                return Some((proposed_report, offset));
            }
        }
    }
    // This part is pretty much unreachable given the input
    None
}

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    // let ss = fs::read_to_string("./temp.txt").unwrap();
    // let mut lines = ss.lines().map(String::from);

    let mut reports = Vec::new();
    let mut pair_maps = Vec::new();

    while let Some(report) = parse_report(&mut lines) {
        pair_maps.push(pair_dists(&report));
        reports.push(report);
    }

    // Initial approach is calling the report fixing routine for every unfixed reports
    // and for every fixed base reports until there's a pair of base and unfixed reports
    // so that the routine can return the fixed report (the routine is already very slow).
    //
    // Optimization idea: only call the routine on pairs of reports that seems to have
    // 12 points in common.
    // What came to mind is that two reports with at least 12 points in common implies that
    // they can create at least 12 choose 2 = 66 overlapping pairs of points. It's not always
    // true the other way around, but the routine is there to check its correctness.
    //
    // Now that the suitable pairs of reports are at hand, the next problem is to traverse
    // these pairs in a more optimized manner than simple brute force. Checking the list of
    // pairs in one input, I realized that 0 can be paired with 6, 7 and 16 and came up with
    // using DFS over a graph with reports as nodes and pairs as edges.

    // Adjacency lists
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..pair_maps.len() - 1 {
        for j in i + 1..pair_maps.len() {
            if overlapped_pair_count(&pair_maps[i], &pair_maps[j]) >= 66 {
                adj.entry(i).or_insert_with(Vec::new).push(j);
                adj.entry(j).or_insert_with(Vec::new).push(i);
            }
        }
    }

    // All "absolute" positions will be expressed relative to scanner 0
    // Array of coordinates of the scanners
    let mut scanners = vec![(0, 0, 0)];
    // Hashmap of fixed reports
    let mut fixed = HashMap::new();
    fixed.insert(0, reports[0].clone());

    // The outer while loop is not neccessary, but it's left there just in case
    while fixed.len() < reports.len() {
        let mut stack = vec![0];
        while !stack.is_empty() {
            let i = stack.pop().unwrap();
            if let Some(neighbors) = adj.get(&i) {
                for j in neighbors.iter() {
                    let j = *j;
                    if fixed.contains_key(&j) {
                        continue;
                    }

                    let base = fixed.get(&i).unwrap();
                    let unfixed = &reports[j];
                    let cands = match_candidates(&pair_maps[j], &pair_maps[i]);

                    if let Some((fixed_report, scanner)) = fix(&unfixed, &base, &cands) {
                        fixed.insert(j, fixed_report);
                        scanners.push(scanner);
                        stack.push(j);
                    }
                }
            }
        }
    }

    // Number of beacons is the number of unique points in all fixed reports
    let points: HashSet<_> = fixed.values().flat_map(|b| b.iter()).collect();
    println!("Part 1: {}", points.len());

    // Largest Manhattan distance between any two scanners
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

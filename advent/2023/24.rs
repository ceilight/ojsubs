use std::io::{self, BufRead};

type Point = (i64, i64, i64);

#[derive(Clone, Copy, Debug)]
struct Stone {
    base: Point,
    delta: Point,
}

impl Stone {
    fn parse(s: &str) -> Stone {
        let (l, r) = s.split_once(" @ ").unwrap();

        fn parse_sect(s: &str) -> Point {
            let mut s = s.split(", ").map(|c| c.parse::<i64>().unwrap());
            (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
        }
        Stone {
            base: parse_sect(l),
            delta: parse_sect(r),
        }
    }
}

fn main() {
    let stones: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| Stone::parse(&l))
        .collect();
    println!("Part 1: {:?}", part1(&stones));
    println!("Part 2: {:?}", part2(&stones));
}

fn slope_and_y_intercept(s: &Stone) -> (f64, f64) {
    let slope = s.delta.1 as f64 / s.delta.0 as f64;
    let y_intercept = s.base.1 as f64 - slope * s.base.0 as f64;
    (slope, y_intercept)
}

fn intersect_xy(l: &Stone, r: &Stone) -> Option<(f64, f64)> {
    let (a, b) = slope_and_y_intercept(l);
    let (c, d) = slope_and_y_intercept(r);
    if a == c {
        return None;
    }
    let x = (d - b) / (a - c);
    let intersect = (x, a * x + b);

    let to_cross = (l.delta.0 > 0) != (l.base.0 as f64 > intersect.0)
        && (l.delta.1 > 0) != (l.base.1 as f64 > intersect.1)
        && (r.delta.0 > 0) != (r.base.0 as f64 > intersect.0)
        && (r.delta.1 > 0) != (r.base.1 as f64 > intersect.1);

    to_cross.then_some(intersect)
}

fn part1(stones: &[Stone]) -> usize {
    let mut count = 0;
    const LOW: f64 = 200000000000000.0;
    const HIGH: f64 = 400000000000000.0;

    for (i, a) in stones.iter().enumerate() {
        for b in stones.iter().skip(i + 1) {
            if let Some((x, y)) = intersect_xy(a, b) {
                if (LOW..=HIGH).contains(&x) && (LOW..=HIGH).contains(&y) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn gaussian_elimination(matrix: &mut [Vec<f64>]) -> Vec<f64> {
    let size = matrix.len();
    assert_eq!(size, matrix[0].len() - 1);

    for i in 0..size - 1 {
        for j in i..size - 1 {
            echelon(matrix, i, j);
        }
    }
    for i in (1..size).rev() {
        eliminate(matrix, i);
    }

    let mut result: Vec<f64> = vec![0.0; size];
    for i in 0..size {
        result[i] = matrix[i][size] / matrix[i][i];
    }
    result
}

fn echelon(matrix: &mut [Vec<f64>], i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] != 0.0 {
        let factor = matrix[j + 1][i] / matrix[i][i];
        (i..size + 1).for_each(|k| {
            matrix[j + 1][k] -= factor * matrix[i][k];
        });
    }
}

fn eliminate(matrix: &mut [Vec<f64>], i: usize) {
    let size = matrix.len();
    if matrix[i][i] != 0.0 {
        for j in (1..i + 1).rev() {
            let factor = matrix[j - 1][i] / matrix[i][i];
            for k in (0..size + 1).rev() {
                matrix[j - 1][k] -= factor * matrix[i][k];
            }
        }
    }
}

#[rustfmt::skip]
fn part2(stones: &[Stone]) -> f64 {
    // Suppose the rock has a starting position P and the velocity vector V;
    // the initial position, velocity vector and the time of collision with the rock
    // of the i-th hailstone are p[i], v[i] and t[i] respectively
    //
    // Then at the time t[i]: P(t[i]) == p[i](t[i])
    // => P + V * t[i] == p[i] + v[i] * t[i], rearrange this and we have:
    // => P - p[i] = -t[i] * (V - v[i])
    // Since t[i] is a scalar, vectors (P - p[i]) and (V - v[i]) are parallel, which
    // means the cross product between them must be zero:
    // => (P - p[i]) * (V - v[i]) = 0
    // => P * V - P * v[i] - p[i] * V + p[i] * v[i] = 0
    // => P * V + v[i] * P - p[i] * V = v[i] * p[i]
    // The resulting equation is bilinear, but you can introduce T = P * V in the 1st term
    // => T + v[i] * P - p[i] * V = v[i] * p[i]
    // and it becomes a linear system of 9 unknowns (with 3 dummy ones T)

    // This is just proof-of-concept, I actually used Python to solve this part :^)
    let mut matrix = vec![];

    for Stone { base, delta } in &stones[..3] {
        let (px, py, pz) = (base.0 as f64, base.1 as f64, base.2 as f64);
        let (vx, vy, vz) = (delta.0 as f64, delta.1 as f64, delta.2 as f64);
        matrix.push(vec![1.0, 0.0, 0.0, 0.0,  vz, -vy, 0.0, -pz,  py, vz * py - vy * pz]);
        matrix.push(vec![0.0, 1.0, 0.0, -vz, 0.0,  vx,  pz, 0.0, -px, vx * pz - vz * px]);
        matrix.push(vec![0.0, 0.0, 1.0,  vy, -vx, 0.0, -py,  px, 0.0, vy * px - vx * py]);
    }

    let result = gaussian_elimination(&mut matrix);

    // The order of the result should be Tx, Ty, Tz, Px, Py, Pz, Vx, Vy, Vz
    result[3] + result[4] + result[5]
}

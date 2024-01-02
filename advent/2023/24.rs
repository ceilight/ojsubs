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

// Last implementation of row reduction gave very strange result for some reason
// so I'm rewriting this step-by-step
#[allow(clippy::needless_range_loop)]
fn solve_linear_system(matrix: &[Vec<f64>]) -> Option<Vec<f64>> {
    let mut m = matrix.to_vec();
    let size = m.len();
    assert_eq!(size, m[0].len() - 1);

    let mut row = 0;
    for col in 0..size {
        // find first non-zero row
        let mut nxt = size;
        for i in row..size {
            if m[i][col] != 0.0 {
                nxt = i;
                break;
            }
        }
        if nxt == size {
            return None;
        }
        // swap it with the current row
        m.swap(nxt, row);

        // rescaling the current row
        let scalar = m[row][col];
        for j in 0..=size {
            m[row][j] /= scalar;
        }

        // subtract other rows a scalar multiple of current row.
        for i in 0..size {
            if i == row {
                continue;
            }
            let scalar = m[i][col];
            for j in 0..=size {
                m[i][j] -= scalar * m[row][j];
            }
        }
        row += 1;
    }

    let mut result: Vec<f64> = vec![0.0; size];
    for i in 0..size {
        result[i] = m[i][size];
    }

    Some(result)
}

#[allow(dead_code)]
#[rustfmt::skip]
fn test() {
    let x = vec![vec![1.0, 3.0, 5.0],
                 vec![2.0, 4.0, 8.0]];
    println!("{:?}", solve_linear_system(&x).unwrap());
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
    // => P * V - P * v[i] - p[i] * V + p[i] * v[i] == 0
    // => P * V + v[i] * P - p[i] * V = v[i] * p[i]
    // The resulting equation is bilinear, but you can introduce K = P * V in the 1st term
    // => K + v[i] * P - p[i] * V == v[i] * p[i]
    // and it becomes a linear system of 9 unknowns (with 3 dummy K(Kx, Ky, Kz))

    // This is just proof-of-concept so the result might be off due to handling of
    // floating-point numbers (it worked on my input however)
    let mut matrix = vec![];

    for Stone { base, delta } in &stones[..3] {
        let (px, py, pz) = (base.0 as f64, base.1 as f64, base.2 as f64);
        let (vx, vy, vz) = (delta.0 as f64, delta.1 as f64, delta.2 as f64);
        matrix.push(vec![1.0, 0.0, 0.0, 0.0,  vz, -vy, 0.0, -pz,  py, vz * py - vy * pz]);
        matrix.push(vec![0.0, 1.0, 0.0, -vz, 0.0,  vx,  pz, 0.0, -px, vx * pz - vz * px]);
        matrix.push(vec![0.0, 0.0, 1.0,  vy, -vx, 0.0, -py,  px, 0.0, vy * px - vx * py]);
    }

    let result = solve_linear_system(&matrix).unwrap();

    // The order of the result should be Kx, Ky, Kz, Px, Py, Pz, Vx, Vy, Vz
    result[3] + result[4] + result[5]
}

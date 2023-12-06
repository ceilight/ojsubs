use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    let line = lines.next().unwrap();
    let times = line.split(':').nth(1).unwrap();
    let times_clone = times;
    let line = lines.next().unwrap();
    let records = line.split(':').nth(1).unwrap();
    let records_clone = records;

    // part 1
    let times = times.split_whitespace().flat_map(str::parse::<f64>);
    let records = records.split_whitespace().flat_map(str::parse::<f64>);
    let p1_ans: f64 = times
        .zip(records)
        .map(|(t, r)| count_ways(t, r))
        .filter(|&c| c > 0.0)
        .product();
    println!("Part 1: {:?}", p1_ans);

    // part 2
    let time = times_clone
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let record = records_clone
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    println!("Part 2: {:?}", count_ways(time, record));
}

fn count_ways(time: f64, record: f64) -> f64 {
    // The problem is finding range in [0, time] so that every integer x of which
    // satisfies the condition x * (time - x) > record, which leads to solving
    // the quadratic inequality: f(x) = x^2 - time*x + record < 0
    // Given the coefficient of x^2 = 1 > 0, and the solutions (if exist) are l and r (l <= r),
    // the x range that satisfies f(x) < 0 is (l, r)
    match solve_quadratic(1.0, -time, record) {
        Some((r1, r2)) => {
            let mut r1_strict = r1.ceil().max(0.0);
            if r1_strict == r1 {
                r1_strict += 1.0
            }
            let mut r2_strict = r2.floor().min(record);
            if r2_strict == r2 {
                r2_strict -= 1.0;
            }
            r2_strict - r1_strict + 1.0
        }
        None => 0.0,
    }
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        return None;
    }
    let delta_sqrt = delta.sqrt();
    let r1 = (-b + delta_sqrt) / (2.0 * a);
    let r2 = (-b - delta_sqrt) / (2.0 * a);
    Some((r1.min(r2), r1.max(r2)))
}

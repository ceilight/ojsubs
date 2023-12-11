use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let grid: Vec<_> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("Part 1: {}", solve(&grid, 1));
    println!("Part 2: {}", solve(&grid, 999999));
}

fn solve(grid: &[String], expand_offset: usize) -> isize {
    let col = grid[0].len();
    let mut rows_after_expansion = HashMap::new();
    let mut expand_rate = 0;

    for (i, r) in grid.iter().enumerate() {
        rows_after_expansion.insert(i + expand_rate, r.chars().collect::<Vec<_>>());
        if r.chars().all(|x| x == '.') {
            expand_rate += expand_offset;
        }
    }

    expand_rate = 0;
    let mut galaxy = Vec::new();
    for j in 0..col {
        galaxy.extend(
            rows_after_expansion
                .iter()
                .filter(|(_, r)| r[j] == '#')
                .map(|(&i, _)| (i, j + expand_rate)),
        );
        if rows_after_expansion.iter().all(|(_, r)| r[j] == '.') {
            expand_rate += expand_offset;
        }
    }

    let mut res = 0;
    for (i, a) in galaxy.iter().enumerate() {
        for (j, b) in galaxy.iter().enumerate() {
            if i < j {
                res += (a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()
            }
        }
    }
    res
}

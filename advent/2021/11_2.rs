use std::io::{self, BufRead};

fn main() {
    let mut grid = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut k = 0;
    loop {
        if grid.iter().all(|c| c.iter().all(|p| *p == 0)) {
            break;
        }
        k += 1;

        let mut pending = vec![];
        let (r, c) = (grid.len(), grid[0].len());
        for i in 0..r {
            for j in 0..c {
                grid[i][j] += 1;
                if grid[i][j] == 10 {
                    pending.push((i, j));
                }
            }
        }

        while !pending.is_empty() {
            let (i, j) = pending.pop().unwrap();
            let (u, d, l, r) = (i > 0, i < r - 1, j > 0, j < c - 1);
            for adj in [
                u.then(|| (i - 1, j)),
                d.then(|| (i + 1, j)),
                l.then(|| (i, j - 1)),
                r.then(|| (i, j + 1)),
                (u && l).then(|| (i - 1, j - 1)),
                (u && r).then(|| (i - 1, j + 1)),
                (d && l).then(|| (i + 1, j - 1)),
                (d && r).then(|| (i + 1, j + 1)),
            ]
            .into_iter()
            .flatten()
            {
                grid[adj.0][adj.1] += 1;
                if grid[adj.0][adj.1] == 10 {
                    pending.push(adj);
                }
            }
        }

        for i in 0..r {
            for j in 0..c {
                if grid[i][j] >= 10 {
                    grid[i][j] = 0;
                }
            }
        }
    }
    println!("{:?}", k);
}

use std::io::{self, BufRead};

fn main() {
    let board = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut lows = vec![];
    let r = board.len();
    let c = board[0].len();

    for i in 0..r {
        for j in 0..c {
            let v = board[i][j];
            if [
                (i > 0).then(|| (i - 1, j)),
                (i < r - 1).then(|| (i + 1, j)),
                (j > 0).then(|| (i, j - 1)),
                (j < c - 1).then(|| (i, j + 1)),
            ]
            .into_iter()
            .flat_map(|p| p)
            .all(|(x, y)| v < board[x][y])
            {
                lows.push((i, j))
            }
        }
    }

    let mut visited = vec![vec![false; c]; r];

    let mut areas: Vec<usize> = lows
        .into_iter()
        .map(|low| {
            let mut pending = vec![low];
            let mut count = 0;

            while !pending.is_empty() {
                let (i, j) = pending.pop().unwrap();
                if visited[i][j] {
                    continue;
                }
                visited[i][j] = true;
                count += 1;

                for adj in [
                    (i > 0).then(|| (i - 1, j)),
                    (i < r - 1).then(|| (i + 1, j)),
                    (j > 0).then(|| (i, j - 1)),
                    (j < c - 1).then(|| (i, j + 1)),
                ]
                .into_iter()
                .flat_map(|p| p)
                {
                    if board[adj.0][adj.1] != 9 && board[adj.0][adj.1] >= board[i][j] {
                        pending.push(adj);
                    }
                }
            }
            count
        })
        .collect();

    areas.sort();
    let ans: usize = areas.iter().rev().take(3).product();
    println!("{:?}", ans);
}

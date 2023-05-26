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

    let mut sum = 0;
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
                sum += v + 1;
            }
        }
    }
    println!("{:?}", sum);
}

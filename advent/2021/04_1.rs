use std::io::{self, BufRead};

struct Board(Vec<Vec<(u32, bool)>>);

impl Board {
    fn mark(&mut self, n: u32) {
        for row in self.0.iter_mut() {
            for (num, called) in row.iter_mut() {
                if *num == n {
                    *called = true;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        for row in self.0.iter() {
            if row.iter().all(|(_, called)| *called == true) {
                return true;
            }
        }
        for i in 0..self.0[0].len() {
            if self.0.iter().all(|row| row[i].1 == true) {
                return true;
            }
        }
        false
    }

    fn get_score(&self) -> u32 {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|(_, called)| *called == false)
                    .map(|(number, _)| *number)
                    .sum::<u32>()
            })
            .sum()
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let sequence: Vec<u32> = lines[0]
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];
    let mut base = 2usize;
    loop {
        let mut rows: Vec<Vec<(u32, bool)>> = vec![];
        for i in base..base + 5 {
            let row: Vec<(u32, bool)> = lines[i]
                .split_whitespace()
                .map(|x| (x.parse::<u32>().unwrap(), false))
                .collect();
            rows.push(row);
        }
        boards.push(Board(rows));
        if base + 6 >= lines.len() {
            break;
        }
        base += 6;
    }

    for num in sequence {
        for board in boards.iter_mut() {
            board.mark(num);
        }
        for i in 0..boards.len() {
            if boards[i].is_bingo() {
                println!("{}", num * boards[i].get_score());
                return;
            }
        }
    }
}

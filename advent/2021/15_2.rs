use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};

#[derive(Eq, PartialEq)]
struct State {
    pos: (usize, usize),
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, rhs: &Self) -> Ordering {
        rhs.cost.cmp(&self.cost).then(self.pos.cmp(&rhs.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}

fn main() {
    let tmp = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut b = Vec::with_capacity(tmp.len() * 5);

    for i in 0u32..5u32 {
        b.extend(tmp.iter().map(|row| {
            let mut extended_row = Vec::with_capacity(row.len() * 5);
            for j in 0u32..5u32 {
                extended_row.extend(row.iter().map(|e| {
                    let x = *e + i + j;
                    if x > 9 {
                        x - 9
                    } else {
                        x
                    }
                }));
            }
            extended_row
        }));
    }

    let (r, c) = (b.len(), b[0].len());
    let start = (0, 0);
    let dest = (r - 1, c - 1);

    let mut pq = BinaryHeap::new();
    let mut cost = HashMap::new();
    pq.push(State {
        pos: start,
        cost: 0,
    });
    cost.insert(start, b[0][0]);

    while !pq.is_empty() {
        let cur = pq.pop().unwrap();
        if cur.pos == dest {
            println!("{:?}", cur.cost);
            return;
        }

        if cur.cost > cost[&cur.pos] {
            continue;
        }

        let (i, j) = cur.pos;
        for adj in [
            (i > 0).then(|| (i - 1, j)),
            (i < r - 1).then(|| (i + 1, j)),
            (j > 0).then(|| (i, j - 1)),
            (j < c - 1).then(|| (i, j + 1)),
        ]
        .into_iter()
        .flatten()
        {
            let next_cost = cur.cost + b[adj.0][adj.1];
            if let Some(c) = cost.get(&adj) {
                if next_cost >= *c {
                    continue;
                }
            }
            pq.push(State {
                pos: adj,
                cost: next_cost,
            });
            cost.insert(adj, next_cost);
        }
    }
}

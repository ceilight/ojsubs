use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn perpendicular_dirs(self) -> Vec<Dir> {
        use Dir::*;
        match self {
            U | D => vec![L, R],
            L | R => vec![U, D],
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct State {
    pos: (usize, usize),
    dir: Dir,
    streak_len: usize,
    cost: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let grid: Vec<Vec<usize>> = lines
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    println!("Part 1: {:?}", solve(&grid, 0, 3));
    println!("Part 2: {:?}", solve(&grid, 4, 10));
}

fn solve(grid: &[Vec<usize>], min: usize, max: usize) -> usize {
    let (row, col) = (grid.len(), grid[0].len());
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(State {
        pos: (0, 0),
        dir: Dir::R,
        streak_len: 0,
        cost: 0,
    });
    queue.push(State {
        pos: (0, 0),
        dir: Dir::D,
        streak_len: 0,
        cost: 0,
    });

    while let Some(state) = queue.pop() {
        let (x, y) = state.pos;
        if x == row - 1 && y == col - 1 && state.streak_len >= min {
            return state.cost;
        }
        if !visited.insert((x, y, state.dir, state.streak_len)) {
            continue;
        }

        let mut next_dirs = vec![];
        if state.streak_len >= min {
            next_dirs.extend(state.dir.perpendicular_dirs());
        }
        if state.streak_len < max {
            next_dirs.push(state.dir);
        }

        for dir in next_dirs {
            if let Some((x, y)) = match dir {
                Dir::U => (x > 0).then_some((x - 1, y)),
                Dir::D => (x < row - 1).then_some((x + 1, y)),
                Dir::L => (y > 0).then_some((x, y - 1)),
                Dir::R => (y < col - 1).then_some((x, y + 1)),
                _ => None,
            } {
                let streak_len = if dir == state.dir {
                    state.streak_len + 1
                } else {
                    1
                };

                queue.push(State {
                    pos: (x, y),
                    dir: dir,
                    streak_len,
                    cost: state.cost + grid[x][y],
                });
            }
        }
    }
    unreachable!();
}

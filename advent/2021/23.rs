// rewriting day 19's solution for the meme (part 2 only)

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};

const COORDS: [(u8, u8); 23] = [
    (0, 1),
    (0, 2),
    (0, 4),
    (0, 6),
    (0, 8),
    (0, 10),
    (0, 11),
    (1, 3),
    (2, 3),
    (3, 3),
    (4, 3),
    (1, 5),
    (2, 5),
    (3, 5),
    (4, 5),
    (1, 7),
    (2, 7),
    (3, 7),
    (4, 7),
    (1, 9),
    (2, 9),
    (3, 9),
    (4, 9),
];

fn cell_index_at_coord(coord: (u8, u8)) -> Option<usize> {
    COORDS.iter().position(|&c| c == coord)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    E = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'A' => Cell::A,
            'B' => Cell::B,
            'C' => Cell::C,
            'D' => Cell::D,
            '.' => Cell::E,
            c => panic!("invalid char {:?}", c),
        }
    }
}

impl Cell {
    fn target_column(&self) -> u8 {
        const TARGET_COLS: [u8; 5] = [0, 3, 5, 7, 9];
        match self {
            Cell::E => panic!("guard this case yourself"),
            c => TARGET_COLS[*c as usize],
        }
    }

    fn weight(&self) -> u32 {
        const WEIGHTS: [u32; 5] = [0, 1, 10, 100, 1000];
        WEIGHTS[*self as usize]
    }
}

#[derive(Clone, Debug)]
struct State {
    cells: Vec<Cell>,
    cost: u32,
}

#[derive(Debug)]
struct Move {
    orig_idx: usize,
    dest_idx: usize,
    cost: u32,
}

impl State {
    fn key(&self) -> u64 {
        let mut res = 0;
        for &c in self.cells.iter() {
            res = 5 * res + c as u64;
        }
        res
    }

    fn get_move(&self, u_idx: usize, v: (u8, u8)) -> Option<Move> {
        assert!(u_idx < 24);
        let u = COORDS[u_idx];
        let dist = (v.0 as i8 - u.0 as i8).abs() as u8 + (v.1 as i8 - u.1 as i8).abs() as u8;
        let u_cell = self.cells[u_idx];
        let cost = dist as u32 * u_cell.weight();
        if let Some(dest_idx) = cell_index_at_coord(v) {
            return Some(Move {
                orig_idx: u_idx,
                dest_idx,
                cost,
            });
        }
        None
    }

    fn apply_move(&self, m: &Move) -> State {
        let mut cells = self.cells.clone();
        cells[m.dest_idx] = cells[m.orig_idx];
        cells[m.orig_idx] = Cell::E;
        State {
            cells,
            cost: self.cost + m.cost,
        }
    }

    fn is_hallway_segment_empty(&self, idx: usize, col_a: u8, col_b: u8) -> bool {
        let col_range = col_a.min(col_b)..=col_a.max(col_b);
        for (i, cell) in self.cells.iter().enumerate() {
            let (row, col) = COORDS[i];
            if row == 0 && col_range.contains(&col) && i != idx && *cell != Cell::E {
                return false;
            }
        }
        true
    }

    fn cell_at_coord(&self, coord: (u8, u8)) -> Option<Cell> {
        if let Some(idx) = cell_index_at_coord(coord) {
            return Some(self.cells[idx]);
        }
        None
    }

    fn available_moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        'next_cell: for (i, cell) in self.cells.iter().enumerate() {
            if *cell == Cell::E {
                continue;
            }
            let (row, col) = COORDS[i];

            if row == 0 {
                // Hallway to target room transition
                let dest_col = cell.target_column();
                let mut dest_row = 0;

                // Iterate over the target room from the bottom up to find the
                // target cell.
                // Check if there are amphipods of other type inside the room
                // so they won't get stuck when the amphipod moves in target room
                for x in (1..=4).rev() {
                    match &self.cell_at_coord((x, dest_col)) {
                        Some(Cell::E) => {
                            dest_row = x;
                            break;
                        }
                        Some(c) if c != cell => continue 'next_cell,
                        Some(_) => (),
                        None => panic!("how is this not a cell"),
                    };
                }

                // Also, make sure the hallway segment between the amphipod and the
                // target room is empty before moving
                assert_ne!(dest_row, 0);
                if !self.is_hallway_segment_empty(i, col, dest_col) {
                    continue;
                }
                moves.push(self.get_move(i, (dest_row, dest_col)).unwrap());
            } else {
                // Room to hallway transition
                // Amphipod can move to the hallway when there are no amphipods overhead
                for x in 1..row {
                    match self.cell_at_coord((x, col)) {
                        Some(Cell::E) => (),
                        Some(_) => continue 'next_cell,
                        None => panic!("how is this not a cell"),
                    };
                }

                // If it's already in the target room, it only moves out when it has
                // to make way for amphipods underneath that are in the wrong room to
                // move to their target rooms
                if col == cell.target_column() {
                    let mut all_comrades = true;
                    for x in row + 1..=4 {
                        match &self.cell_at_coord((x, col)) {
                            Some(Cell::E) => panic!("they're not gonna bite you bro"),
                            Some(c) if c != cell => {
                                all_comrades = false;
                                break;
                            }
                            Some(_) => (),
                            None => panic!("how is this not a cell"),
                        }
                    }
                    if all_comrades {
                        continue 'next_cell;
                    }
                }

                // Also, make sure the hallway segment between the amphipod and the
                // target room is empty before moving
                const HALLWAY_COLS: [u8; 7] = [1, 2, 4, 6, 8, 10, 11];
                for &y in HALLWAY_COLS.iter() {
                    if self.is_hallway_segment_empty(i, col, y) {
                        moves.push(self.get_move(i, (0, y)).unwrap());
                    }
                }
            }
        }
        moves
    }

    fn equal_cells_with(&self, other: &Self) -> bool {
        self.cells
            .iter()
            .zip(other.cells.iter())
            .all(|(a, b)| a == b)
    }
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

fn dijskstra(init: &State, end: &State) -> Option<(Vec<State>, u32)> {
    let mut costs: HashMap<u64, u32> = HashMap::new();
    let mut prevs: HashMap<u64, State> = HashMap::new();

    let mut queue = BinaryHeap::new();
    queue.push(init.clone());

    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        let cost = state.cost;

        if state.equal_cells_with(end) {
            let mut path = vec![];
            let mut cur = state;
            loop {
                let prev = prevs.get(&cur.key());
                path.push(cur);
                match prev {
                    Some(p) => cur = p.clone(),
                    None => break,
                };
            }
            path.reverse();
            return Some((path, cost));
        }

        let moves = state.available_moves();
        for m in moves.into_iter() {
            let next = state.apply_move(&m);
            let k = next.key();
            match costs.get(&k) {
                Some(&c) if next.cost >= c => continue,
                _ => {
                    costs.insert(k, next.cost);
                    prevs.insert(k, state.clone());
                }
            };
            queue.push(next);
        }
    }
    None
}

fn parse_input<I>(mut lines: I) -> State
where
    I: Iterator<Item = String>,
{
    lines.next();
    let grid = lines
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cells: Vec<_> = (0..23)
        .map(|i| {
            let (x, y) = COORDS[i];
            Cell::from(grid[x as usize][y as usize])
        })
        .collect();
    State { cells, cost: 0 }
}

fn main() {
    // use the input that includes 2 additional lines
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    let init_state = parse_input(&mut lines);

    let end_state = {
        use Cell::*;
        State {
            cells: vec![
                E, E, E, E, E, E, E, A, A, A, A, B, B, B, B, C, C, C, C, D, D, D, D,
            ],
            cost: 0,
        }
    };
    println!("{:?}", dijskstra(&init_state, &end_state).unwrap().1);
}

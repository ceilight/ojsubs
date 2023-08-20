use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};
use std::time::Instant;

const CELLS_ROW: [i8; 23] = [
    0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4,
];
const CELLS_COL: [i8; 23] = [
    1, 2, 4, 6, 8, 10, 11, 3, 3, 3, 3, 5, 5, 5, 5, 7, 7, 7, 7, 9, 9, 9, 9,
];

fn cell_coord(idx: usize) -> (i8, i8) {
    (CELLS_ROW[idx], CELLS_COL[idx])
}

fn cell_index_at_coord(coord: (i8, i8)) -> Option<usize> {
    (0..23).find(|&i| CELLS_ROW[i] == coord.0 && CELLS_COL[i] == coord.1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn target_column(&self) -> i8 {
        const TARGET_COLS: [i8; 5] = [0, 3, 5, 7, 9];
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

#[derive(Clone, Debug)]
struct Move {
    orig_idx: usize,
    dest_idx: usize,
    cost: u32,
}

impl State {
    fn parse_grid(grid: Vec<Vec<char>>) -> Self {
        let room_depth = grid.len() as i8 - 2;
        let mut cells = vec![Cell::E; 23];
        for (i, c) in cells.iter_mut().enumerate() {
            let (x, y) = cell_coord(i);
            if x > 0 && x <= room_depth {
                *c = Cell::from(grid[x as usize][y as usize]);
            }
        }
        State { cells, cost: 0 }
    }

    // Since a cell can be one of 5 values (A, B, C, D, empty), the cells attribute
    // can be hashed in a base 5 system number
    fn cells_hash(&self) -> u64 {
        self.cells.iter().fold(0, |hash, &c| hash * 5 + c as u64)
    }

    fn amphipod_count(&self) -> usize {
        self.cells.iter().filter(|&&x| x != Cell::E).count()
    }

    fn get_move(&self, u_idx: usize, v: (i8, i8)) -> Option<Move> {
        assert!(u_idx < 23);
        let u = cell_coord(u_idx);
        let dist = (v.0 - u.0).abs() + (v.1 - u.1).abs();
        cell_index_at_coord(v).map(|dest_idx| Move {
            orig_idx: u_idx,
            dest_idx,
            cost: dist as u32 * self.cells[u_idx].weight(),
        })
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

    fn is_hallway_segment_empty(&self, idx: usize, col_a: i8, col_b: i8) -> bool {
        let col_range = col_a.min(col_b)..=col_a.max(col_b);
        (0..self.cells.len())
            .filter(|&i| {
                let (r, c) = cell_coord(i);
                r == 0 && col_range.contains(&c) && i != idx
            })
            .all(|i| self.cells[i] == Cell::E)
    }

    fn cell_at_coord(&self, coord: (i8, i8)) -> Option<Cell> {
        cell_index_at_coord(coord).map(|idx| self.cells[idx])
    }

    fn available_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let row_max = self.amphipod_count() as i8 / 4;

        'next_cell: for (i, &cell) in self.cells.iter().enumerate() {
            if cell == Cell::E {
                continue;
            }
            let (row, col) = cell_coord(i);

            if row == 0 {
                // Hallway to target room transition
                let dest_col = cell.target_column();
                let mut dest_row = 0;

                // Iterate over the target room from the bottom up to find the target cell.
                // Check if there are amphipods of other type inside the room
                // so they won't get stuck when the amphipod moves in target room
                for x in (1..=row_max).rev() {
                    match self.cell_at_coord((x, dest_col)) {
                        Some(Cell::E) => {
                            dest_row = x;
                            break;
                        }
                        Some(c) if c != cell => continue 'next_cell,
                        Some(_) => (),
                        None => panic!("how is {:?} not a cell", (x, dest_col)),
                    };
                }

                // Also, make sure the hallway segment between the amphipod and the
                // target room is empty before moving
                assert_ne!(dest_row, 0);
                if self.is_hallway_segment_empty(i, col, dest_col) {
                    moves.push(self.get_move(i, (dest_row, dest_col)).unwrap());
                }
            } else {
                // Room to hallway transition
                // Amphipod can move to the hallway when there are no amphipods overhead
                for x in 1..row {
                    match self.cell_at_coord((x, col)) {
                        Some(Cell::E) => (),
                        Some(_) => continue 'next_cell,
                        None => panic!("how is {:?} not a cell", (x, col)),
                    };
                }

                // If it's already in the target room, it only moves out when it has
                // to make way for amphipods underneath that are in the wrong room to
                // move to their target rooms
                if col == cell.target_column() {
                    let mut all_comrades = true;
                    for x in row + 1..=row_max {
                        match self.cell_at_coord((x, col)) {
                            Some(Cell::E) => panic!("they're not gonna bite you bro"),
                            Some(c) if c != cell => {
                                all_comrades = false;
                                break;
                            }
                            Some(_) => (),
                            None => panic!("how is {:?} not a cell", (x, col)),
                        }
                    }
                    if all_comrades {
                        continue;
                    }
                }

                const HALLWAY_COLS: [i8; 7] = [1, 2, 4, 6, 8, 10, 11];
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

fn dijkstra(init: &State, end: &State) -> Option<(Vec<Move>, u32)> {
    let mut costs: HashMap<u64, u32> = HashMap::new();
    let mut prevs: HashMap<u64, (u64, Move)> = HashMap::new();

    let mut queue = BinaryHeap::new();
    queue.push(init.clone());

    while let Some(state) = queue.pop() {
        // if end state is reached, trace move path and return total cost
        if state.equal_cells_with(end) {
            let mut path = vec![];
            let init_hash = init.cells_hash();
            let mut hash = state.cells_hash();
            while hash != init_hash {
                let (h, m) = prevs.get(&hash).unwrap();
                path.push(m.clone());
                hash = *h;
            }
            path.reverse();
            return Some((path, state.cost));
        }

        let moves = state.available_moves();
        for m in moves.iter() {
            let next = state.apply_move(m);
            let next_key = next.cells_hash();
            match costs.get(&next_key) {
                Some(&c) if next.cost >= c => continue,
                _ => {
                    costs.insert(next_key, next.cost);
                    prevs.insert(next_key, (state.cells_hash(), m.clone()));
                }
            };
            queue.push(next);
        }
    }
    None
}

fn main() {
    use Cell::*;

    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    lines.next();
    let grid1: Vec<Vec<_>> = lines.map(|l| l.chars().collect()).collect();
    let mut grid2 = grid1.clone();

    let init1 = State::parse_grid(grid1);
    let end1 = State {
        cells: vec![
            E, E, E, E, E, E, E, A, A, E, E, B, B, E, E, C, C, E, E, D, D, E, E,
        ],
        cost: 0,
    };

    // add 2 more lines for part 2
    grid2.insert(2, "  #D#C#B#A#".chars().collect());
    grid2.insert(3, "  #D#B#A#C#".chars().collect());
    let init2 = State::parse_grid(grid2);
    let end2 = State {
        cells: vec![
            E, E, E, E, E, E, E, A, A, A, A, B, B, B, B, C, C, C, C, D, D, D, D,
        ],
        cost: 0,
    };

    let start = Instant::now();
    let (_p, c) = dijkstra(&init1, &end1).unwrap();
    let p1_time = start.elapsed();
    // for m in p.iter() {
    //     println!("{:?} -> {:?} ", cell_coord(m.orig_idx), cell_coord(m.dest_idx));
    // }
    println!("Part 1: {} ({}ms elapsed)", c, p1_time.as_millis());
    let start = Instant::now();
    let (_p, c) = dijkstra(&init2, &end2).unwrap();
    let p2_time = start.elapsed();
    // for m in p.iter() {
    //     println!("{:?} -> {:?} ", cell_coord(m.orig_idx), cell_coord(m.dest_idx));
    // }
    println!("Part 2: {} ({}ms elapsed)", c, p2_time.as_millis());
}

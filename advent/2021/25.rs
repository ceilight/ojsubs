use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Loc {
    Void,
    East,
    South,
}

impl From<char> for Loc {
    fn from(c: char) -> Self {
        match c {
            '.' => Loc::Void,
            '>' => Loc::East,
            'v' => Loc::South,
            c => panic!("invalid char {}", c),
        }
    }
}

struct World {
    board: Vec<Vec<Loc>>,
    rows: usize,
    cols: usize,
    advance_count: u32,
}

impl World {
    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let board: Vec<Vec<Loc>> = lines.map(|l| l.chars().map(Loc::from).collect()).collect();
        assert!(board.len() > 1);
        let (rows, cols) = (board.len(), board[0].len());
        Self {
            board,
            rows,
            cols,
            advance_count: 0,
        }
    }

    fn advance(&mut self) -> bool {
        self.advance_count += 1;
        let mut changed = false;

        let mut to_void = Vec::new();
        let mut to_move = Vec::new();
        for x in 0..self.rows {
            for y in 0..self.cols {
                if self.board[x][y] == Loc::East {
                    let yn = (y + 1) % self.cols;
                    if self.board[x][yn] == Loc::Void {
                        to_void.push((x, y));
                        to_move.push((x, yn));
                    }
                }
            }
        }
        if !to_move.is_empty() {
            changed = true;
        }
        for (x, y) in to_void.iter() {
            self.board[*x][*y] = Loc::Void;
        }
        for (x, y) in to_move.iter() {
            self.board[*x][*y] = Loc::East;
        }

        let mut to_void = Vec::new();
        let mut to_move = Vec::new();
        for y in 0..self.cols {
            for x in 0..self.rows {
                if self.board[x][y] == Loc::South {
                    let xn = (x + 1) % self.rows;
                    if self.board[xn][y] == Loc::Void {
                        to_void.push((x, y));
                        to_move.push((xn, y));
                    }
                }
            }
        }
        if !to_move.is_empty() {
            changed = true;
        }
        for (x, y) in to_void.iter() {
            self.board[*x][*y] = Loc::Void;
        }
        for (x, y) in to_move.iter() {
            self.board[*x][*y] = Loc::South;
        }
        changed
    }

    #[allow(dead_code)]
    fn debug(&self) {
        for r in self.board.iter() {
            for e in r.iter() {
                print!(
                    "{}",
                    match *e {
                        Loc::Void => '.',
                        Loc::East => '>',
                        Loc::South => 'v',
                    }
                );
            }
            println!("");
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let mut world = World::parse(lines);
    let count = loop {
        if !world.advance() {
            break world.advance_count;
        }
    };
    println!("{}", count);
}

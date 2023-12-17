use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let grid: Vec<Vec<_>> = lines.map(|l| l.chars().collect()).collect();
    println!("Part 1: {:?}", part1(&grid));
    println!("Part 2: {:?}", part2(&grid));
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn to_mask(self) -> u8 {
        use Dir::*;
        match self {
            U => 1,
            D => 1 << 1,
            L => 1 << 2,
            R => 1 << 3,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Forward(Dir),
    Split(Dir, Dir),
}

fn next_move(tile: char, dir: Dir) -> Move {
    use Dir::*;
    use Move::*;
    match (tile, dir) {
        ('/', U) => Forward(R),
        ('/', D) => Forward(L),
        ('/', L) => Forward(D),
        ('/', R) => Forward(U),
        ('\\', U) => Forward(L),
        ('\\', D) => Forward(R),
        ('\\', L) => Forward(U),
        ('\\', R) => Forward(D),
        ('-', U | D) => Split(L, R),
        ('|', L | R) => Split(U, D),
        _ => Forward(dir),
    }
}

fn part1(grid: &[Vec<char>]) -> usize {
    energize(grid, 0, 0, Dir::R)
}

fn part2(grid: &[Vec<char>]) -> usize {
    let (row, col) = (grid.len(), grid[0].len());
    let row_start = (0..row).flat_map(|r| [(r, 0, Dir::R), (r, col - 1, Dir::L)]);
    let col_start = (1..col - 1).flat_map(|c| [(0, c, Dir::D), (row - 1, c, Dir::U)]);
    row_start
        .chain(col_start)
        .map(|(x, y, d)| energize(grid, x, y, d))
        .max()
        .unwrap()
}

fn energize(grid: &[Vec<char>], x: usize, y: usize, dir: Dir) -> usize {
    let (row, col) = (grid.len(), grid[0].len());
    let mut beams = vec![(x, y, dir)];
    let mut tile_dirs = vec![0u8; row * col];
    tile_dirs[x * col + y] |= dir.to_mask();

    while let Some((x, y, dir)) = beams.pop() {
        let mut apply_move = |dir: Dir| {
            if let Some((x, y)) = match dir {
                Dir::U => (x > 0).then_some((x - 1, y)),
                Dir::D => (x < row - 1).then_some((x + 1, y)),
                Dir::L => (y > 0).then_some((x, y - 1)),
                Dir::R => (y < col - 1).then_some((x, y + 1)),
            } {
                let ds = &mut tile_dirs[x * col + y];
                if *ds & dir.to_mask() == 0 {
                    *ds |= dir.to_mask();
                    beams.push((x, y, dir));
                }
            }
        };

        match next_move(grid[x][y], dir) {
            Move::Forward(d) => apply_move(d),
            Move::Split(a, b) => {
                apply_move(a);
                apply_move(b);
            }
        }
    }
    tile_dirs.iter().filter(|&&i| i != 0).count()
}

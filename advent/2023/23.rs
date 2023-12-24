use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let (row, col, mut tiles) = parse_input(lines);

    // part 1
    println!("Part 1: {:?}", solve(row, col, &tiles));
    // part 2
    for (_, v) in tiles.iter_mut() {
        if matches!(v, Tile::Slope(_)) {
            *v = Tile::Path;
        }
    }
    println!("Part 2: {:?}", solve(row, col, &tiles));
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Path,
    Slope(Dir),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Path,
            '^' => Tile::Slope(Dir::U),
            'v' => Tile::Slope(Dir::D),
            '<' => Tile::Slope(Dir::L),
            '>' => Tile::Slope(Dir::R),
            _ => panic!("invalid tile {:?}", c),
        }
    }
}

type Point = (usize, usize);

fn parse_input(lines: impl Iterator<Item = String>) -> (usize, usize, HashMap<Point, Tile>) {
    let grid: Vec<_> = lines.map(|l| l.chars().collect::<Vec<_>>()).collect();
    let (row, col) = (grid.len(), grid[0].len());

    let mut tiles = HashMap::new();
    for (x, r) in grid.iter().enumerate() {
        for (y, &c) in r.iter().enumerate() {
            if c != '#' {
                tiles.insert((x, y), Tile::from(c));
            }
        }
    }
    (row, col, tiles)
}

#[derive(Copy, Clone, Debug)]
struct Adj {
    point: Point,
    distance: usize,
}

type Graph = HashMap<Point, Vec<Adj>>;

fn construct_graph(row: usize, col: usize, tiles: &HashMap<Point, Tile>) -> Graph {
    let mut graph: Graph = HashMap::new();

    for &(x, y) in tiles.keys() {
        let e = graph.entry((x, y)).or_default();

        match tiles.get(&(x, y)).unwrap() {
            Tile::Path => {
                for (nx, ny) in [
                    x.checked_sub(1).map(|x| (x, y)),
                    (x < row - 1).then_some((x + 1, y)),
                    y.checked_sub(1).map(|y| (x, y)),
                    (y < col - 1).then_some((x, y + 1)),
                ]
                .iter()
                .flatten()
                .filter(|p| tiles.contains_key(p))
                {
                    e.push(Adj {
                        point: (*nx, *ny),
                        distance: 1,
                    });
                }
            }
            Tile::Slope(dir) => {
                if let Some((nx, ny)) = match dir {
                    Dir::U => x.checked_sub(1).map(|x| (x, y)),
                    Dir::D => (x < row - 1).then_some((x + 1, y)),
                    Dir::L => y.checked_sub(1).map(|y| (x, y)),
                    Dir::R => (y < col - 1).then_some((x, y + 1)),
                } {
                    if tiles.contains_key(&(nx, ny)) {
                        e.push(Adj {
                            point: (nx, ny),
                            distance: 1,
                        });
                    }
                }
            }
        }
    }

    // compress the graph by removing 2-degree vertices
    let to_remove: Vec<_> = graph
        .iter()
        .filter(|(_, v)| v.len() == 2) // starting point and target are 1-degree vertices
        .map(|(&p, _)| p)
        .collect();

    for p in to_remove {
        let adjacents = graph.remove(&p).unwrap();
        let (u, v) = (adjacents[0], adjacents[1]);
        let u_adj = graph.get_mut(&(u.point)).unwrap();
        if let Some(i) = u_adj.iter().position(|&x| x.point == p) {
            u_adj[i].point = v.point;
            u_adj[i].distance += v.distance;
        }

        let v_adj = graph.get_mut(&(v.point)).unwrap();
        if let Some(i) = v_adj.iter().position(|&x| x.point == p) {
            v_adj[i].point = u.point;
            v_adj[i].distance += u.distance;
        }
    }

    graph
}

fn dfs(
    graph: &Graph,
    curr: Point,
    end: Point,
    visited: &mut HashMap<Point, bool>,
) -> Option<usize> {
    if curr == end {
        return Some(0);
    }

    let mut max_distance = None;
    for Adj { point, distance } in graph.get(&curr).unwrap() {
        let seen = visited.entry(*point).or_insert(false);
        if !*seen {
            *seen = true;
            if let Some(d) = dfs(graph, *point, end, visited) {
                max_distance = Some(max_distance.unwrap_or(0).max(d + distance));
            }
            visited.entry(*point).and_modify(|x| *x = false);
        }
    }

    max_distance
}

fn solve(row: usize, col: usize, tiles: &HashMap<Point, Tile>) -> usize {
    let graph = construct_graph(row, col, tiles);
    let start = graph.keys().find(|&p| p.0 == 0).unwrap();
    let end = graph.keys().find(|&p| p.0 == row - 1).unwrap();

    dfs(&graph, *start, *end, &mut HashMap::new()).unwrap()
}

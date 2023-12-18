use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    dir: Dir,
    num: usize,
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &[String]) -> isize {
    let plan = lines.iter().map(|l| {
        let mut sp = l.split(' ');
        let dir = sp.next().unwrap().chars().next().unwrap();
        let dir = match dir {
            'U' => Dir::U,
            'D' => Dir::D,
            'L' => Dir::L,
            'R' => Dir::R,
            _ => panic!("invalid char: {:?}", dir),
        };
        let num = sp.next().unwrap().parse::<usize>().unwrap();
        Instruction { dir, num }
    });

    wait_this_is_just_day_10_p2(plan)
}

fn part2(lines: &[String]) -> isize {
    let plan = lines.iter().map(|l| {
        let hex = l.split(' ').nth(2).unwrap();
        let num = usize::from_str_radix(&hex[2..hex.len() - 2], 16).unwrap();
        let dir = hex.chars().nth(hex.len() - 2).unwrap();
        let dir = match dir {
            '0' => Dir::R,
            '1' => Dir::D,
            '2' => Dir::L,
            '3' => Dir::U,
            _ => panic!("invald digit: {:?}", dir),
        };
        Instruction { dir, num }
    });

    wait_this_is_just_day_10_p2(plan)
}

fn wait_this_is_just_day_10_p2(plan: impl Iterator<Item = Instruction>) -> isize {
    let mut vertices = vec![];
    let (mut x, mut y) = (0_isize, 0_isize);

    for instr in plan {
        vertices.push((x, y));
        match instr.dir {
            Dir::U => x -= instr.num as isize,
            Dir::D => x += instr.num as isize,
            Dir::L => y -= instr.num as isize,
            Dir::R => y += instr.num as isize,
        };
    }

    let mut peri = 0_isize;
    let mut area = 0_isize;
    for idx in 0..vertices.len() {
        let next_idx = (idx + 1) % vertices.len();
        let (u, v) = (vertices[idx], vertices[next_idx]);
        peri += (u.0 - v.0).abs() + (u.1 - v.1).abs();
        area += u.0 * v.1 - u.1 * v.0;
    }
    (area.abs() - peri) / 2 + 1 + peri
}

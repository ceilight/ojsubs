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

    // only correct if the trench edges do not form a self-intersecting loop
    // so that the depth of all blocks in the trench surface is 1
    trench_area(plan)
}

fn part2(lines: &[String]) -> isize {
    let plan = lines.iter().map(|l| {
        let hex = &l[l.find('#').unwrap() + 1..l.find(')').unwrap()];
        let num = usize::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();
        let dir = hex.chars().last().unwrap();
        let dir = match dir {
            '0' => Dir::R,
            '1' => Dir::D,
            '2' => Dir::L,
            '3' => Dir::U,
            _ => panic!("invald digit: {:?}", dir),
        };
        Instruction { dir, num }
    });

    trench_area(plan)
}

// shoelace formula can be applied here like day 10's solution
// here i use green's theorem + pick's theorem
fn trench_area(plan: impl Iterator<Item = Instruction>) -> isize {
    let (mut x, mut y) = (0_isize, 0_isize);
    let mut area = 0_isize;
    let mut peri = 0_isize;

    for instr in plan {
        let num = instr.num as isize;
        peri += num;
        let (dx, dy) = match instr.dir {
            Dir::U => (-num, 0),
            Dir::D => (num, 0),
            Dir::L => (0, -num),
            Dir::R => (0, num),
        };
        area += x * dy - y * dx;
        x += dx;
        y += dy;
    }
    assert!(x == 0 && y == 0);

    (area.abs() + peri) / 2 + 1
}

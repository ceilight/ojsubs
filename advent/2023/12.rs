use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter;

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let data: Vec<_> = lines
        .map(|line| {
            let mut line = line.split(' ');
            let springs: Vec<char> = line.next().unwrap().chars().collect();
            let groups = line
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            (springs, groups)
        })
        .collect();

    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
}

fn part1(data: &[(Vec<char>, Vec<usize>)]) -> usize {
    data.iter()
        .map(|(springs, groups)| calc_arrangements(springs, groups, &mut HashMap::new()))
        .sum()
}

fn part2(data: &[(Vec<char>, Vec<usize>)]) -> usize {
    let data = data.iter().map(|(springs, groups)| {
        let (rlen, glen) = (springs.len(), groups.len());

        // replace the list of spring conditions with five copies of itself (separated by ?)
        // >(separated by ?)
        let springs: Vec<_> = springs
            .iter()
            .copied()
            .chain(iter::once('?'))
            .cycle()
            .take(rlen * 5 + 4)
            .collect();
        let groups: Vec<_> = groups.iter().cycle().take(glen * 5).copied().collect();
        (springs, groups)
    });

    data.map(|(springs, groups)| calc_arrangements(&springs, &groups, &mut HashMap::new()))
        .sum()
}

fn calc_arrangements(
    springs: &[char],
    groups: &[usize],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if springs.is_empty() {
        if groups.is_empty() {
            return 1;
        }
        return 0;
    }
    if groups.is_empty() {
        if springs.contains(&'#') {
            return 0;
        }
        return 1;
    }

    // using springs and groups slice at each call as state seems expensive
    // and the content of springs and groups isn't changing anyway, so it's a
    // better idea to make state a pair of usize values indicating the numbers of
    // springs and groups remaining
    let state = (springs.len(), groups.len());
    if let Some(calc) = memo.get(&state) {
        return *calc;
    }

    let mut res = 0;
    match springs[0] {
        '.' => {
            let next_springs = &springs[1.min(springs.len())..];
            res += calc_arrangements(next_springs, groups, memo);
        }
        '#' if is_possible_group(springs, groups[0]) => {
            let springs_index = (groups[0] + 1).min(springs.len());
            let next_springs = &springs[springs_index..];
            let next_groups = &groups[1.min(groups.len())..];
            res += calc_arrangements(next_springs, next_groups, memo);
        }
        '?' => {
            // unknown report is '.'
            let next_springs = &springs[1.min(springs.len())..];
            res += calc_arrangements(next_springs, groups, memo);
            // unknown report is '#'
            if is_possible_group(springs, groups[0]) {
                let springs_index = (groups[0] + 1).min(springs.len());
                let next_springs = &springs[springs_index..];
                let next_groups = &groups[1.min(groups.len())..];
                res += calc_arrangements(next_springs, next_groups, memo);
            }
        }
        _ => (),
    }

    memo.insert(state, res);
    res
}

fn is_possible_group(springs: &[char], len: usize) -> bool {
    if springs.len() < len {
        return false;
    }
    if springs[..len].contains(&'.') {
        return false;
    }
    if springs.len() > len && springs[len] == '#' {
        return false;
    }
    true
}

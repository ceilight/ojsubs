use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter;

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let data: Vec<_> = lines
        .map(|line| {
            let mut line = line.split(' ');
            let records = line.next().unwrap();
            let records: Vec<char> = records.chars().collect();
            let group_lens = line.next().unwrap();
            let group_lens: Vec<_> = group_lens
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            (records, group_lens)
        })
        .collect();

    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
}

fn part1(data: &[(Vec<char>, Vec<usize>)]) -> usize {
    let mut res = 0;
    for (records, group_lens) in data {
        // not really necessary for part 1, but it'd be fatal solving part 2 w/o memoization
        let mut memo = HashMap::new();
        res += calc_arrangements(records, group_lens, &mut memo);
    }
    res
}

fn part2(data: &[(Vec<char>, Vec<usize>)]) -> usize {
    let data = data.iter().map(|(r, g)| {
        let (rlen, glen) = (r.len(), g.len());
        // replace the list of spring conditions with five copies of itself (separated by ?)
        // >(separated by ?)
        let r: Vec<_> = r
            .iter()
            .chain(iter::once(&'?'))
            .cycle()
            .take(rlen * 5 + 4)
            .copied()
            .collect();
        let g: Vec<_> = g.iter().cycle().take(glen * 5).copied().collect();
        (r, g)
    });

    let mut res = 0;
    for (records, group_lens) in data {
        let mut memo = HashMap::new();
        res += calc_arrangements(&records, &group_lens, &mut memo);
    }
    res
}

fn calc_arrangements<'a>(
    records: &'a [char],
    group_lens: &'a [usize],
    memo: &mut HashMap<(&'a [char], &'a [usize]), usize>,
) -> usize {
    if records.is_empty() {
        if group_lens.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }
    if group_lens.is_empty() {
        if records.contains(&'#') {
            return 0;
        } else {
            return 1;
        }
    }

    let unknown_count = records.iter().filter(|&&x| x == '?').count();
    let damaged_count = records.iter().filter(|&&x| x == '#').count();
    let total_count: usize = group_lens.iter().sum();
    if unknown_count + damaged_count < total_count {
        return 0;
    }

    if let Some(calc) = memo.get(&(records, group_lens)) {
        return *calc;
    }

    let mut res = 0;
    match records[0] {
        '.' => {
            let next_records = &records[1.min(records.len())..];
            res += calc_arrangements(next_records, group_lens, memo);
        }
        '#' if is_possible_group(records, group_lens[0]) => {
            let records_index = (group_lens[0] + 1).min(records.len());
            let next_records = &records[records_index..];
            let next_group_lens = &group_lens[1.min(group_lens.len())..];
            res += calc_arrangements(next_records, next_group_lens, memo);
        }
        '?' => {
            let next_records = &records[1.min(records.len())..];
            res += calc_arrangements(next_records, group_lens, memo);
            if is_possible_group(records, group_lens[0]) {
                let records_index = (group_lens[0] + 1).min(records.len());
                let next_records = &records[records_index..];
                let next_group_lens = &group_lens[1.min(group_lens.len())..];
                res += calc_arrangements(next_records, next_group_lens, memo);
            }
        }
        _ => (),
    }

    memo.insert((records, group_lens), res);
    res
}

fn is_possible_group(records: &[char], len: usize) -> bool {
    if records[..len].contains(&'.') {
        return false;
    }
    if records.len() > len && records[len] == '#' {
        return false;
    }
    true
}

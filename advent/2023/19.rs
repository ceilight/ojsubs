use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let (flows, test) = parse_input(lines);
    println!("Part 1: {:?}", part1(&flows, &test));
    println!("Part 2: {:?}", part2(&flows));
}

#[derive(Clone, Debug)]
enum Expr {
    Accepted,
    Rejected,
    Redirect(String),
}

impl From<&str> for Expr {
    fn from(raw: &str) -> Self {
        match raw {
            "A" => Expr::Accepted,
            "R" => Expr::Rejected,
            raw if !raw.is_empty() => Expr::Redirect(String::from(raw)),
            _ => panic!("empty workflow label"),
        }
    }
}

#[derive(Clone, Debug)]
enum Switch {
    Less(usize, usize, Expr),
    More(usize, usize, Expr),
}

#[derive(Clone, Debug)]
struct Workflow {
    switches: Vec<Switch>,
    default: Expr,
}

impl From<&str> for Workflow {
    fn from(flow: &str) -> Self {
        let (switches_raw, default_raw) = flow.rsplit_once(',').unwrap();
        let switches: Vec<_> = switches_raw
            .split(',')
            .map(|s| {
                let (cond, expr) = s.split_once(':').unwrap();
                let op_idx = cond.find(['>', '<']).unwrap();
                let var = match &cond[..op_idx] {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    c => panic!("invalid variable {:?}", c),
                };
                let val: usize = cond[op_idx + 1..].parse().unwrap();
                let expr = Expr::from(expr);
                match cond.as_bytes()[op_idx] {
                    b'<' => Switch::Less(var, val, expr),
                    b'>' => Switch::More(var, val, expr),
                    c => panic!("invalid char {:?}", c),
                }
            })
            .collect();
        let default = Expr::from(default_raw);

        Workflow { switches, default }
    }
}

type WorkflowMap = HashMap<String, Workflow>;

fn parse_input(mut lines: impl Iterator<Item = String>) -> (WorkflowMap, Vec<Vec<usize>>) {
    let mut flows = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let l_end = line.find('{').unwrap();
        let r_end = line.find('}').unwrap();
        let label = String::from(&line[..l_end]);
        let flow = Workflow::from(&line[l_end + 1..r_end]);
        flows.insert(label, flow);
    }

    let test: Vec<_> = lines
        .map(|line| {
            let line = &line[line.find('{').unwrap() + 1..line.find('}').unwrap()];
            line.split(',')
                .map(|x| x[2..].parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (flows, test)
}

fn check_nums(flows: &WorkflowMap, expr: &Expr, nums: &[usize]) -> bool {
    let label = match expr {
        Expr::Accepted => return true,
        Expr::Rejected => return false,
        Expr::Redirect(l) => l,
    };

    let Workflow { switches, default } = flows.get(label).unwrap();
    for s in switches {
        match s {
            Switch::Less(i, v, e) if nums[*i] < *v => return check_nums(flows, e, nums),
            Switch::More(i, v, e) if nums[*i] > *v => return check_nums(flows, e, nums),
            _ => (),
        }
    }
    check_nums(flows, default, nums)
}

fn part1(flows: &WorkflowMap, test: &[Vec<usize>]) -> usize {
    test.iter()
        .flat_map(|n| {
            (check_nums(flows, &Expr::Redirect("in".to_owned()), n))
                .then_some(n.iter().sum::<usize>())
        })
        .sum()
}

fn find_accepted_ranges(
    flows: &WorkflowMap,
    expr: &Expr,
    mut num_ranges: [(usize, usize); 4], // copy not reference
) -> Vec<[(usize, usize); 4]> {
    if num_ranges.iter().any(|(l, r)| l >= r) {
        return vec![];
    }
    let label = match expr {
        Expr::Accepted => return vec![num_ranges],
        Expr::Rejected => return vec![],
        Expr::Redirect(l) => l,
    };

    let mut valid_ranges = vec![];

    let Workflow { switches, default } = flows.get(label).unwrap();
    for s in switches {
        let (idx, success_range, fail_range, next_expr) = match s {
            Switch::Less(i, v, e) => {
                let r = num_ranges[*i];
                (*i, (r.0, *v), (*v, r.1), e)
            }
            Switch::More(i, v, e) => {
                let r = num_ranges[*i];
                (*i, (*v + 1, r.1), (r.0, *v + 1), e)
            }
        };
        num_ranges[idx] = success_range;
        valid_ranges.extend(find_accepted_ranges(flows, next_expr, num_ranges));
        num_ranges[idx] = fail_range;
    }
    valid_ranges.extend(find_accepted_ranges(flows, default, num_ranges));

    valid_ranges
}

fn part2(flows: &WorkflowMap) -> usize {
    let init_expr = Expr::Redirect("in".to_owned());
    let ranges = find_accepted_ranges(flows, &init_expr, [(1, 4001); 4]);
    ranges
        .iter()
        .map(|r| r.iter().map(|(l, r)| r - l).product::<usize>())
        .sum()
}

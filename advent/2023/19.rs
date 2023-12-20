use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(Result::unwrap).collect();
    let (flows, ratings) = parse_input(&lines);
    println!("Part 1: {:?}", part1(&flows, &ratings));
    println!("Part 2: {:?}", part2(&flows));
}

#[derive(Clone, Debug)]
enum Expr<'a> {
    Accepted,
    Rejected,
    Redirect(&'a str),
}

impl<'a> From<&'a str> for Expr<'a> {
    fn from(s: &'a str) -> Self {
        match s {
            "A" => Expr::Accepted,
            "R" => Expr::Rejected,
            s if !s.is_empty() => Expr::Redirect(s),
            _ => panic!("empty workflow label"),
        }
    }
}

#[derive(Clone, Debug)]
enum Switch<'a> {
    Less(usize, usize, Expr<'a>),
    More(usize, usize, Expr<'a>),
    Dflt(Expr<'a>),
}

impl<'a> Switch<'a> {
    fn parse(s: &'a str) -> Self {
        if let Some((cond, expr)) = s.split_once(':') {
            let var_idx = |c: &str| -> usize {
                match c {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => panic!("invalid variable {:?}", c),
                }
            };
            let expr = Expr::from(expr);
            if let Some((var, val)) = cond.split_once('<') {
                Switch::Less(var_idx(var), val.parse().unwrap(), expr)
            } else if let Some((var, val)) = cond.split_once('>') {
                Switch::More(var_idx(var), val.parse().unwrap(), expr)
            } else {
                unreachable!();
            }
        } else {
            Switch::Dflt(Expr::from(s))
        }
    }
}

type WorkflowMap<'a> = HashMap<&'a str, Vec<Switch<'a>>>;

fn parse_input(lines: &[String]) -> (WorkflowMap, Vec<Vec<usize>>) {
    let mut flows = HashMap::new();
    let mut lines = lines.iter();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (label, flow) = line.strip_suffix('}').unwrap().split_once('{').unwrap();
        let flow: Vec<_> = flow.split(',').map(Switch::parse).collect();
        flows.insert(label, flow);
    }

    let ratings: Vec<_> = lines
        .map(|line| {
            let line = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
            line.split(',')
                .map(|s| s[2..].parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (flows, ratings)
}

fn check_rating<'a>(flows: &WorkflowMap<'a>, expr: &Expr<'a>, rating: &[usize]) -> bool {
    let label = match expr {
        Expr::Accepted => return true,
        Expr::Rejected => return false,
        Expr::Redirect(l) => l,
    };

    let flow = flows.get(label).unwrap();
    for s in flow {
        match s {
            Switch::Less(i, v, e) if rating[*i] < *v => return check_rating(flows, e, rating),
            Switch::More(i, v, e) if rating[*i] > *v => return check_rating(flows, e, rating),
            Switch::Dflt(e) => return check_rating(flows, e, rating),
            _ => (),
        }
    }
    unreachable!();
}

fn part1(flows: &WorkflowMap, ratings: &[Vec<usize>]) -> usize {
    ratings
        .iter()
        .flat_map(|n| {
            (check_rating(flows, &Expr::Redirect("in"), n)).then_some(n.iter().sum::<usize>())
        })
        .sum()
}

fn count_combinations(ranges: [(usize, usize); 4]) -> usize {
    ranges.iter().map(|(l, r)| r - l).product()
}

fn count_accepted_ratings<'a>(
    flows: &WorkflowMap<'a>,
    expr: &Expr<'a>,
    mut ranges: [(usize, usize); 4], // copy not reference
) -> usize {
    if ranges.iter().any(|(l, r)| l >= r) {
        return 0;
    }
    let label = match expr {
        Expr::Accepted => return count_combinations(ranges),
        Expr::Rejected => return 0,
        Expr::Redirect(l) => l,
    };

    let mut count = 0;

    let flow = flows.get(label).unwrap();
    for s in flow {
        match s {
            Switch::Less(i, val, next) => {
                let range = ranges[*i];
                ranges[*i] = (range.0, *val);
                count += count_accepted_ratings(flows, next, ranges);
                ranges[*i] = (*val, range.1);
            }
            Switch::More(i, val, next) => {
                let range = ranges[*i];
                ranges[*i] = (*val + 1, range.1);
                count += count_accepted_ratings(flows, next, ranges);
                ranges[*i] = (range.0, *val + 1);
            }
            Switch::Dflt(next) => {
                count += count_accepted_ratings(flows, next, ranges);
            }
        };
    }

    count
}

fn part2(flows: &WorkflowMap) -> usize {
    let init_expr = Expr::Redirect("in");
    count_accepted_ratings(flows, &init_expr, [(1, 4001); 4])
}

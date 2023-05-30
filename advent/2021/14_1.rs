use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();

    let template = lines.next().unwrap().unwrap().chars().collect::<Vec<_>>();
    let mut pairs = HashMap::new();
    for x in template.windows(2) {
        *pairs.entry((x[0], x[1])).or_insert(0) += 1;
    }
    lines.next();

    let rules = lines
        .map(|l| {
            let l = l.unwrap();
            let mut sp = l.split(" -> ");
            let mut u = sp.next().unwrap().chars();
            let mut v = sp.next().unwrap().chars();
            ((u.next().unwrap(), u.next().unwrap()), v.next().unwrap())
        })
        .collect::<HashMap<_, _>>();

    for _ in 0..10 {
        let change = pairs
            .iter()
            .map(|(p, n)| {
                if rules.contains_key(&p) {
                    Some((p.clone(), *n, rules[&p]))
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        for (p, n, r) in change {
            *pairs.get_mut(&p).unwrap() -= n;
            *pairs.entry((p.0, r)).or_insert(0) += n;
            *pairs.entry((r, p.1)).or_insert(0) += n;
        }
        pairs.retain(|_, n| *n > 0);
    }

    let mut count = HashMap::new();
    for (p, n) in pairs.into_iter() {
        *count.entry(p.0).or_insert(0) += n;
        *count.entry(p.1).or_insert(0) += n;
    }

    *count.entry(template[0]).or_insert(0) += 1;
    *count.entry(template[template.len() - 1]).or_insert(0) += 1;

    let count_max = count.iter().map(|(_, n)| *n).max().unwrap();
    let count_min = count.iter().map(|(_, n)| *n).min().unwrap();

    println!("{:?}", (count_max - count_min) / 2);
}

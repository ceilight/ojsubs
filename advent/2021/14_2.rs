use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();

    let template = lines.next().unwrap().unwrap().chars().collect::<Vec<_>>();
    let mut pairs = HashMap::new();
    for x in template.windows(2) {
        *pairs.entry((x[0], x[1])).or_insert(0usize) += 1;
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

    for _ in 0..40 {
        pairs = pairs
            .into_iter()
            .map(|(p, n)| {
                if rules.contains_key(&p) {
                    let t = rules.get(&p).unwrap();
                    let (l, r) = ((p.0, *t), (*t, p.1));
                    Vec::from([(l, n), (r, n)])
                } else {
                    Vec::from([(p, n)])
                }
            })
            .flatten()
            .fold(HashMap::new(), |mut h, (p, n)| {
                *h.entry(p).or_insert(0usize) += n;
                h
            });
    }

    let mut count = HashMap::new();
    for (p, n) in pairs.into_iter() {
        *count.entry(p.0).or_insert(0usize) += n;
        *count.entry(p.1).or_insert(0usize) += n;
    }

    *count.entry(template[0]).or_insert(0usize) += 1;
    *count.entry(template[template.len() - 1]).or_insert(0usize) += 1;

    let count_max = count.iter().map(|(_, n)| *n).max().unwrap();
    let count_min = count.iter().map(|(_, n)| *n).min().unwrap();

    println!("{:?}", (count_max - count_min) / 2);
}

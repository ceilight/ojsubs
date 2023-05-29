use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let data = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let mut adjs = HashMap::new();
    for line in data.iter() {
        let mut sp = line.split("-");
        let u = sp.next().unwrap();
        let v = sp.next().unwrap();

        adjs.entry(u).or_insert(vec![]).push(v);
        adjs.entry(v).or_insert(vec![]).push(u);
    }

    let mut count = 0;
    let mut pending = vec![];
    pending.push(("start", HashSet::new()));

    while !pending.is_empty() {
        let (u, s) = pending.pop().unwrap();
        for v in adjs[u].iter() {
            match *v {
                "start" => (),
                "end" => count += 1,
                v if v.chars().all(char::is_lowercase) => {
                    if !s.contains(v) {
                        let mut ss = s.clone();
                        ss.insert(v);
                        pending.push((v, ss));
                    }
                }
                v => pending.push((v, s.clone())),
            };
        }
    }

    println!("{:?}", count);
}

use std::collections::HashMap;
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
    pending.push(("start", HashMap::new()));

    while !pending.is_empty() {
        let (u, s) = pending.pop().unwrap();
        for v in adjs[u].iter() {
            match *v {
                "start" => (),
                "end" => count += 1,
                v if v.chars().all(char::is_lowercase) => match s.get(&v).unwrap_or(&0) {
                    0 => {
                        let mut ss = s.clone();
                        ss.insert(v, 1);
                        pending.push((v, ss));
                    }
                    1 if s.values().all(|c| *c < 2) => {
                        let mut ss = s.clone();
                        ss.insert(v, 2);
                        pending.push((v, ss));
                    }
                    _ => (),
                },
                v => pending.push((v, s.clone())),
            };
        }
    }

    println!("{:?}", count);
}

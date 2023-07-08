use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct N(Vec<(u8, u8)>);

impl N {
    fn concat(&mut self, other: &mut N) {
        self.0.append(&mut other.0);
        self.0.iter_mut().for_each(|(_, d)| *d += 1);
    }

    fn reduce(&mut self, st: usize) {
        for i in st..self.0.len() - 1 {
            let d = self.0[i].1;
            if d >= 5 && d == self.0[i + 1].1 {
                let (l, r) = (self.0[i].0, self.0[i + 1].0);
                self.0[i] = (0, d - 1);
                self.0.remove(i + 1);

                if let Some(v) = self.0.get_mut(i.overflowing_sub(1).0) {
                    (*v).0 += l;
                }
                if let Some(v) = self.0.get_mut(i + 1) {
                    (*v).0 += r;
                }
                return self.reduce(i);
            }
        }
        for i in 0..self.0.len() {
            let (n, d) = self.0[i];
            if n >= 10 {
                self.0[i] = (n / 2, d + 1);
                self.0.insert(i + 1, ((n + 1) / 2, d + 1));
                return self.reduce(i);
            }
        }
    }

    fn mag(&self, i: &mut usize, d: u8) -> u16 {
        let l = if self.0[*i].1 == d {
            *i += 1;
            self.0[*i - 1].0 as u16
        } else {
            self.mag(i, d + 1)
        };

        let r = if self.0[*i].1 == d {
            *i += 1;
            self.0[*i - 1].0 as u16
        } else {
            self.mag(i, d + 1)
        };

        3 * l + 2 * r
    }

    fn add(&mut self, other: &mut N) {
        self.concat(other);
        self.reduce(0);
    }

    fn magnitude(&self) -> u16 {
        self.mag(&mut 0, 1)
    }
}

fn main() {
    let x = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let tmp = line.chars().fold((0, Vec::new()), |(mut d, mut n), c| {
                match c {
                    '[' => d += 1,
                    ']' => d -= 1,
                    '0'..='9' => n.push((c.to_digit(10).unwrap() as u8, d)),
                    _ => (),
                };
                (d, n)
            });
            N(tmp.1)
        })
        .collect::<Vec<_>>();

    let mag_max = (0..x.len() - 1)
        .into_iter()
        .map(|i| {
            (i + 1..x.len())
                .into_iter()
                .map(|j| {
                    let (mut a, mut b) = (x[i].clone(), x[j].clone());
                    let (mut sa, mut sb) = (a.clone(), b.clone());
                    sa.add(&mut b);
                    sb.add(&mut a);
                    let res = sa.magnitude().max(sb.magnitude());
                    res
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{:?}", mag_max);
}

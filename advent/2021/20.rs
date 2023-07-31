use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Light,
    Dark,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Pixel::Light,
            '.' => Pixel::Dark,
            c => panic!("Invalid char {:?}", c),
        }
    }
}

impl Pixel {
    fn bit(self) -> usize {
        match self {
            Pixel::Light => 1,
            Pixel::Dark => 0,
        }
    }
}

struct Img {
    algo: Vec<Pixel>,
    pixels: HashMap<(i16, i16), Pixel>,
    min_x: i16,
    min_y: i16,
    max_x: i16,
    max_y: i16,
    void_state: Pixel,
}

impl Img {
    fn parse<I>(mut lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let algo: Vec<_> = lines.next().unwrap().chars().map(Pixel::from).collect();
        lines.next();

        let mut pixels = HashMap::new();
        let mut max_x = i16::MIN;
        let mut min_x = i16::MAX;
        let mut max_y = i16::MIN;
        let mut min_y = i16::MAX;

        for (y, line) in lines.enumerate() {
            let y = y as i16;
            for (x, c) in line.chars().enumerate() {
                let x = x as i16;
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                pixels.insert((x, y), Pixel::from(c));
            }
        }

        Self {
            algo,
            pixels,
            min_x,
            min_y,
            max_x,
            max_y,
            void_state: Pixel::Dark,
        }
    }

    fn enhance_at(&self, x: i16, y: i16) -> Pixel {
        let mut idx = 0usize;
        for y in y - 1..=y + 1 {
            for x in x - 1..=x + 1 {
                let bit = self.pixels.get(&(x, y)).unwrap_or(&self.void_state).bit();
                idx = (idx << 1) | bit;
            }
        }
        self.algo[idx]
    }

    fn enhance(&mut self) {
        let mut next = HashMap::new();
        for (x, y) in self.pixels.keys() {
            let (x, y) = (*x, *y);
            next.insert((x, y), self.enhance_at(x, y));
        }

        self.min_x -= 1;
        self.min_y -= 1;
        self.max_x += 1;
        self.max_y += 1;

        // pixels bordering the main image get incorporated
        for x in self.min_x..=self.max_x {
            for y in [self.min_y, self.max_y] {
                next.insert((x, y), self.enhance_at(x, y));
            }
        }
        for y in self.min_y + 1..=self.max_y - 1 {
            for x in [self.min_x, self.max_x] {
                next.insert((x, y), self.enhance_at(x, y));
            }
        }
        self.pixels = next;

        // void is dark, 9 pixels are all dark, index is 0
        // void is light, 9 pixels are all lit, index is 511
        self.void_state = match self.void_state {
            Pixel::Dark => self.algo[0],
            Pixel::Light => self.algo[511],
        };
    }

    fn count_lits(&self) -> usize {
        assert!(self.void_state == Pixel::Dark);
        self.pixels
            .iter()
            .filter(|(_, &c)| c == Pixel::Light)
            .count()
    }
}

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    let mut image = Img::parse(&mut lines);
    for _ in 0..2 {
        image.enhance();
    }
    println!("Part 1: {:?}", image.count_lits());
    for _ in 0..48 {
        image.enhance();
    }
    println!("Part 2: {:?}", image.count_lits());
}

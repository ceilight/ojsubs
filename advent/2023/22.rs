use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Copy, Clone, Debug)]
struct Brick {
    lo: Point,
    hi: Point,
}

impl Brick {
    fn parse(s: &str) -> Self {
        let (l, r) = s.split_once('~').unwrap();

        let mut l = l.split(',').map(|v| v.parse::<isize>().unwrap());
        let l = Point {
            x: l.next().unwrap(),
            y: l.next().unwrap(),
            z: l.next().unwrap(),
        };
        let mut r = r.split(',').map(|v| v.parse::<isize>().unwrap());
        let r = Point {
            x: r.next().unwrap(),
            y: r.next().unwrap(),
            z: r.next().unwrap(),
        };

        if l.x != r.x && l.y != r.y && l.z != r.z {
            panic!("this {:?} a staircase", (l, r));
        }

        if l.z > r.z || l.x > r.x || l.y > r.y {
            Brick { lo: r, hi: l }
        } else {
            Brick { lo: l, hi: r }
        }
    }

    fn collides(&self, other: &Self) -> bool {
        (self.lo.x >= other.lo.x && self.lo.x <= other.hi.x)
            && (other.lo.y >= self.lo.y && other.lo.y <= self.hi.y)
            || (self.lo.y >= other.lo.y && self.lo.y <= other.hi.y)
                && (other.lo.x >= self.lo.x && other.lo.x <= self.hi.x)
    }

    fn fall_to(&mut self, z: isize) {
        self.hi.z -= self.lo.z - z;
        self.lo.z = z;
    }
}

fn drop_bricks(bricks: &mut [Brick]) -> usize {
    let mut ans = 0;
    for i in 0..bricks.len() {
        let falling = &bricks[i];
        let z = falling.hi.z;
        let z = if let Some(bot) = bricks[..i]
            .iter()
            .filter(|b| b.hi.z < z && falling.collides(b))
            .max_by_key(|b| b.hi.z)
        {
            bot.hi.z + 1
        } else {
            1
        };

        let falling = &mut bricks[i];
        if falling.lo.z > z {
            falling.fall_to(z);
            ans += 1;
        }
    }
    ans
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let mut bricks: Vec<_> = lines.map(|l| Brick::parse(&l)).collect();
    bricks.sort_by_key(|b| b.lo.z);

    drop_bricks(&mut bricks);
    bricks.sort_by_key(|b| b.lo.z);

    // nvm bruteforcing is enough
    let mut destroyed = 0;
    let mut affected = 0;

    for i in 0..bricks.len() {
        let mut temp = bricks.clone();
        temp.remove(i);
        let x = drop_bricks(&mut temp);
        if x == 0 {
            destroyed += 1;
        } else {
            affected += x;
        }
    }

    println!("Part 1: {:?}", destroyed);
    println!("Part 2: {:?}", affected);
}

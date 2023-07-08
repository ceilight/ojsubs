use std::cmp;
use std::io::{self, BufRead};

fn parse_input() -> ((i32, i32), (i32, i32)) {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let line = line.strip_prefix("target area: ").unwrap();
    let mut spl = line.split(", ");

    let mut parse_coord = || {
        // attempt:
        // > let c = spl.next().unwrap()[2..];
        // error:
        // > [E0277]: the size for values of type `str` cannot be known at compilation time
        // expl:
        // > str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory.
        //   since the size is unknown, one can only handle it behind a pointer, as `&str`
        let c = &spl.next().unwrap()[2..];
        let mut c = c.split("..");
        let x: i32 = c.next().unwrap().parse().unwrap();
        let y: i32 = c.next().unwrap().parse().unwrap();
        (x, y)
    };

    (parse_coord(), parse_coord())
}

fn trajectory_in_target(vx: i32, vy: i32, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> bool {
    let (mut vx, mut vy) = (vx, vy);
    let (mut x, mut y) = (0, 0);
    // acceleration on x-axis due to drag
    let ax = if vx > 0 {
        -1
    } else if vx < 0 {
        1
    } else {
        0
    };
    loop {
        x += vx;
        y += vy;
        if vx != 0 {
            vx += ax;
        }
        vy -= 1;

        let x_flag = (xmin..=xmax).contains(&x);
        let y_flag = (ymin..=ymax).contains(&y);

        if x_flag && y_flag {
            return true;
        }
        if !x_flag && (vx == 0 || (vx > 0 && x > xmax) || (vx < 0 && x < xmin)) {
            return false;
        }
        if y < ymin {
            return false;
        }
    }
}

fn main() {
    let ((xmin, xmax), (ymin, ymax)) = parse_input();

    let vy0 = -ymin - 1;
    println!("Part 1: {:?}", vy0 * (vy0 + 1) / 2);

    let xabs_max = cmp::max(xmin.abs(), xmax.abs());
    let velocity_count: usize = (ymin..-ymin)
        .into_iter()
        .map(|vy| {
            (-xabs_max..=xabs_max)
                .into_iter()
                .filter(|vx| trajectory_in_target(*vx, vy, xmin, xmax, ymin, ymax))
                .count()
        })
        .sum();

    println!("Part 2: {:?}", velocity_count);
}

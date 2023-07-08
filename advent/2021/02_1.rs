use std::io::{self, BufRead};

// #[derive(Debug)]
enum MoveType {
    Up,
    Down,
    Forward,
}

struct Move(MoveType, i32);

impl Move {
    fn from_input(s: &str) -> Self {
        let tokens: Vec<&str> = s.split(" ").collect();
        Self(
            match tokens[0] {
                "forward" => MoveType::Forward,
                "up" => MoveType::Up,
                "down" => MoveType::Down,
                _ => {
                    panic!("?");
                }
            },
            tokens[1].parse::<i32>().unwrap(),
        )
    }
}

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn apply(&mut self, m: &Move) {
        match m.0 {
            MoveType::Forward => self.x += m.1,
            MoveType::Up => self.y -= m.1,
            MoveType::Down => self.y += m.1,
        }
    }
}

fn main() {
    let mut submarine = Point::default();
    for line in io::stdin().lock().lines() {
        let cmd = Move::from_input(&line.unwrap());
        submarine.apply(&cmd);
    }
    println!("{}", submarine.x * submarine.y);
}

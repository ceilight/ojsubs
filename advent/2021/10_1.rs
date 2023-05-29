use std::io::{self, BufRead};

fn main() {
    let tot: u64 = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut st = vec![];

            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => st.push(c),
                    ')' if st.pop() == Some('(') => (),
                    ']' if st.pop() == Some('[') => (),
                    '}' if st.pop() == Some('{') => (),
                    '>' if st.pop() == Some('<') => (),
                    c => return Some(c),
                };
            }
            None
        })
        .flatten()
        .map(|c| {
            match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0, // i had faith in the input
            }
        })
        .sum();

    println!("{:?}", tot);
}

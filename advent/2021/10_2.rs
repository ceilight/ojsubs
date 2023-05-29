use std::io::{self, BufRead};

fn main() {
    let mut scores: Vec<u64> = io::stdin()
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
                    c => return None,
                };
            }

            let mut score = 0;
            for c in st.into_iter().rev() {
                score = score * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    }
            }
            Some(score)
        })
        .flatten()
        .collect();

    scores.sort();
    println!("{:?}", scores[scores.len() / 2]);
}

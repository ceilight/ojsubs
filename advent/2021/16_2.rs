use std::io::{self, BufRead};

fn parse_bits(bits: &[bool], nbit: usize) -> Option<(&[bool], usize)> {
    if bits.len() < nbit {
        return None;
    }
    let mut res = 0;
    for i in 0..nbit {
        res = (res << 1) | bits[i] as usize;
    }
    Some((&bits[nbit..], res))
}

fn parse_literal_packet(mut bits: &[bool]) -> Option<(&[bool], usize)> {
    let mut num = 0;
    loop {
        let cont = bits[0];
        let (b, v) = parse_bits(&bits[1..], 4)?;
        num = (num << 4) | v;
        bits = b;
        if !cont {
            return Some((bits, num));
        }
    }
}

fn parse_operator_packet(
    mut bits: &[bool],
    op: fn(usize, usize) -> usize,
) -> Option<(&[bool], usize)> {
    let (lentype, mut bits) = bits.split_first().unwrap();
    if *lentype {
        let mut tot = 0;
        let (bits, subcnt) = parse_bits(bits, 11)?;
        let (mut bits, mut res) = parse_packet(bits)?;
        for _ in 1..subcnt {
            let (b, v) = parse_packet(bits)?;
            res = op(res, v);
            bits = b;
        }
        Some((bits, res))
    } else {
        let (bits, len) = parse_bits(bits, 15)?;
        let (sub, bits) = bits.split_at(len as usize);
        let (mut sub, mut res) = parse_packet(sub)?;
        while let Some((s, v)) = parse_packet(sub) {
            res = op(res, v);
            sub = s;
        }
        Some((bits, res))
    }
}

fn parse_packet(bits: &[bool]) -> Option<(&[bool], usize)> {
    let (bits, ver) = parse_bits(bits, 3)?;
    let (bits, typ) = parse_bits(bits, 3)?;
    match typ {
        0 => parse_operator_packet(bits, |x, y| x + y),
        1 => parse_operator_packet(bits, |x, y| x * y),
        2 => parse_operator_packet(bits, |x, y| if x < y { x } else { y }),
        3 => parse_operator_packet(bits, |x, y| if x > y { x } else { y }),
        4 => parse_literal_packet(bits),
        5 => parse_operator_packet(bits, |x, y| (x > y) as usize),
        6 => parse_operator_packet(bits, |x, y| (x < y) as usize),
        7 => parse_operator_packet(bits, |x, y| (x == y) as usize),
        _ => panic!("skill issue!"),
    }
}

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let bits: Vec<bool> = line
        .chars()
        .map(|x| {
            let n = x.to_digit(16).unwrap();
            format!("{:04b}", n)
                .chars()
                .map(|x| x == '1')
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    println!("{:?}", parse_packet(&bits).unwrap().1);
}

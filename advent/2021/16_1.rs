use std::io::{self, BufRead};

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

    println!("{:?}", parse_all(&bits));
}

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

fn parse_literal_packet(mut bits: &[bool]) -> &[bool] {
    loop {
        let cont = bits[0];
        bits = &bits[5..];
        if !cont {
            return bits;
        }
    }
}

fn parse_operator_packet(mut bits: &[bool]) -> Option<(&[bool], usize)> {
    let (lentype, mut bits) = match bits.split_first() {
        Some((f, b)) => (f, b),
        None => return Some((bits, 0)),
    };

    if *lentype {
        let mut tot = 0;
        let (mut bits, subcnt) = parse_bits(bits, 11)?;
        for _ in 0..subcnt {
            let (b, v) = parse_packet(bits).unwrap();
            bits = b;
            tot += v;
        }
        Some((bits, tot))
    } else {
        let (bits, len) = parse_bits(bits, 15)?;
        let v = parse_all(&bits[..len]);
        Some((&bits[len..], v))
    }
}

fn parse_packet(bits: &[bool]) -> Option<(&[bool], usize)> {
    let (bits, ver) = parse_bits(bits, 3)?;
    let (bits, typ) = parse_bits(bits, 3)?;
    let (bits, subsum) = if typ == 4 {
        (parse_literal_packet(bits), 0)
    } else {
        parse_operator_packet(bits)?
    };
    Some((bits, subsum + ver))
}

fn parse_all(mut bits: &[bool]) -> usize {
    let mut tot = 0;
    loop {
        if let Some((b, v)) = parse_packet(bits) {
            tot += v;
            bits = b;
        } else {
            return tot;
        }
    }
}

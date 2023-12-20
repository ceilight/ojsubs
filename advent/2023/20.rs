use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

// untyped module does not count as one
#[derive(Clone, Debug)]
enum ModuleKind {
    Broadcast,
    Flipflop,
    Conjuction,
}

#[derive(Clone, Debug)]
struct Module<'a> {
    kind: ModuleKind,
    state: bool,
    receivers: Vec<&'a str>,
}

fn parse_module_config(s: &str) -> (&str, Module) {
    let (label, receivers) = s.split_once(" -> ").unwrap();
    let (name, kind) = match &label[0..1] {
        "b" => (label, ModuleKind::Broadcast),
        "%" => (&label[1..], ModuleKind::Flipflop),
        "&" => (&label[1..], ModuleKind::Conjuction),
        l => panic!("invalid label format {:?}", l),
    };
    let module = Module {
        kind,
        state: false,
        receivers: receivers.split(", ").collect(),
    };
    (name, module)
}

struct Pulse<'a>(&'a str, &'a str, bool);

fn part1(lines: &[String]) -> usize {
    let mut modules: HashMap<_, _> = lines.iter().map(|line| parse_module_config(line)).collect();

    let mut receiving_pulses_map: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    for (v_name, v) in modules.iter() {
        if matches!(v.kind, ModuleKind::Conjuction) {
            for (u_name, u) in modules.iter() {
                if u.receivers.contains(v_name) {
                    receiving_pulses_map
                        .entry(v_name)
                        .or_default()
                        .insert(u_name, false);
                }
            }
        }
    }

    let (mut low_count, mut high_count) = (0, 0);
    for _ in 0..1000 {
        let mut queue: VecDeque<Pulse> = VecDeque::new();
        queue.push_back(Pulse("", "broadcaster", false));
        while let Some(Pulse(sender, curr, is_high)) = queue.pop_front() {
            if is_high {
                high_count += 1;
            } else {
                low_count += 1;
            }

            if let Some(module) = modules.get_mut(curr) {
                if let Some(is_next_high) = match module.kind {
                    ModuleKind::Broadcast => Some(is_high),
                    ModuleKind::Flipflop => {
                        if is_high {
                            None
                        } else {
                            module.state = !module.state;
                            Some(module.state)
                        }
                    }
                    ModuleKind::Conjuction => {
                        let p = receiving_pulses_map.get_mut(curr).unwrap();
                        p.insert(sender, is_high);
                        if p.values().all(|&x| x) {
                            Some(false)
                        } else {
                            Some(true)
                        }
                    }
                } {
                    for receiver in module.receivers.iter() {
                        queue.push_back(Pulse(curr, receiver, is_next_high));
                    }
                }
            }
        }
    }
    low_count * high_count
}

fn part2(lines: &[String]) -> usize {
    let mut modules: HashMap<_, _> = lines.iter().map(|line| parse_module_config(line)).collect();

    let mut receiving_pulses_map: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    for (v_name, v) in modules.iter() {
        if matches!(v.kind, ModuleKind::Conjuction) {
            for (u_name, u) in modules.iter() {
                if u.receivers.contains(v_name) {
                    receiving_pulses_map
                        .entry(v_name)
                        .or_default()
                        .insert(u_name, false);
                }
            }
        }
    }

    // input observation: 'rx' is an untyped module linked to a single conjuction module M
    // and the only way to send a low pulse to 'rx' is to get all the incoming pulses to
    // module M high
    let rx_prev_name = modules
        .iter()
        .find(|(_, m)| m.receivers.contains(&"rx"))
        .unwrap()
        .0;
    let rx_prev_prev: Vec<_> = receiving_pulses_map[rx_prev_name].keys().collect();

    // tracks the rounds where each module observed by M sends a high pulse
    let mut history: HashMap<_, _> = rx_prev_prev.iter().map(|&n| (*n, vec![])).collect();

    // tweak the range until there's output, it just werks :^)
    'rounds: for round in 0..1_000_000_usize {
        let mut queue: VecDeque<Pulse> = VecDeque::new();
        queue.push_back(Pulse("", "broadcaster", false));

        while let Some(Pulse(sender, curr, is_high)) = queue.pop_front() {
            if history.contains_key(sender) && is_high {
                history.entry(sender).and_modify(|x| x.push(round));
                if history.values().all(|v| v.len() >= 2) {
                    break 'rounds;
                }
            }

            if let Some(module) = modules.get_mut(curr) {
                if let Some(is_next_high) = match module.kind {
                    ModuleKind::Broadcast => Some(is_high),
                    ModuleKind::Flipflop => {
                        if is_high {
                            None
                        } else {
                            module.state = !module.state;
                            Some(module.state)
                        }
                    }
                    ModuleKind::Conjuction => {
                        let p = receiving_pulses_map.get_mut(curr).unwrap();
                        p.insert(sender, is_high);
                        if p.values().all(|&x| x) {
                            Some(false)
                        } else {
                            Some(true)
                        }
                    }
                } {
                    for receiver in module.receivers.iter() {
                        queue.push_back(Pulse(curr, receiver, is_next_high));
                    }
                }
            }
        }
    }

    lcm(history.values().map(|v| v[1] - v[0]))
}

fn lcm(mut nums: impl Iterator<Item = usize>) -> usize {
    let mut lcm = if let Some(num) = nums.next() {
        num
    } else {
        return 0;
    };
    for x in nums {
        lcm = lcm / gcd(lcm, x) * x;
    }
    lcm
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

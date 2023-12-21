use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

#[derive(Clone, Copy, Debug, Hash)]
enum Pulse {
    High,
    Low,
}

// untyped module does not count as one
#[derive(Clone, Debug)]
enum Module<'a> {
    Bcast,
    Flip(bool),
    Conj(HashMap<&'a str, Pulse>),
}

type ModuleMap<'a> = HashMap<&'a str, Module<'a>>;
type DestsMap<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(lines: &[String]) -> (ModuleMap<'_>, DestsMap<'_>) {
    let mut module_map: ModuleMap<'_> = HashMap::new();
    let mut dests_map: DestsMap<'_> = HashMap::new();

    for s in lines {
        let (label, dests) = s.split_once(" -> ").unwrap();
        let (name, module) = match &label[0..1] {
            "b" => (label, Module::Bcast),
            "%" => (&label[1..], Module::Flip(false)),
            "&" => (&label[1..], Module::Conj(HashMap::new())),
            l => panic!("invalid label format {:?}", l),
        };
        module_map.insert(name, module);
        dests_map.insert(name, dests.split(", ").collect());
    }

    for (vn, v) in module_map.iter_mut() {
        if let Module::Conj(inp) = v {
            inp.extend(
                dests_map
                    .iter()
                    .filter(|(_, udests)| udests.contains(vn))
                    .map(|(&un, _)| (un, Pulse::Low)),
            );
        }
    }

    (module_map, dests_map)
}

fn press_button<'a, F>(module_map: &mut ModuleMap<'a>, dests_map: &DestsMap<'a>, mut extra: F)
where
    F: FnMut(&'a str, &'a str, Pulse),
{
    let mut queue = VecDeque::new();
    queue.push_back(("", "broadcaster", Pulse::Low));

    while let Some((sender, curr, pulse)) = queue.pop_front() {
        extra(sender, curr, pulse);

        if let Some(module) = module_map.get_mut(curr) {
            let next_pulse = match module {
                Module::Bcast => pulse,
                Module::Flip(state) => match pulse {
                    Pulse::High => continue,
                    Pulse::Low => {
                        let p = if *state { Pulse::Low } else { Pulse::High };
                        *state = !*state;
                        p
                    }
                },
                Module::Conj(inp) => {
                    inp.insert(sender, pulse);
                    if inp.values().all(|&x| matches!(x, Pulse::High)) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    }
                }
            };

            let ns = dests_map.get(curr).unwrap();
            for next in ns.iter() {
                queue.push_back((curr, next, next_pulse));
            }
        }
    }
}

fn part1(lines: &[String]) -> usize {
    let (mut module_map, dests_map) = parse_input(lines);
    let (mut low_count, mut high_count) = (0, 0);

    for _ in 0..1000 {
        press_button(&mut module_map, &dests_map, |_, _, pulse| {
            match pulse {
                Pulse::High => high_count += 1,
                Pulse::Low => low_count += 1,
            };
        });
    }

    low_count * high_count
}

fn part2(lines: &[String]) -> usize {
    let (mut module_map, dests_map) = parse_input(lines);

    // input observation: 'rx' is an untyped module linked to a single conjuction module M
    // and the only way to send a low pulse to 'rx' is to have all incoming pulses to
    // module M to be high ones
    let target_prev = dests_map
        .iter()
        .find(|(_, dests)| dests.contains(&"rx"))
        .unwrap()
        .0;

    let mut history = if let Module::Conj(target_pp) = module_map.get(target_prev).unwrap() {
        target_pp
            .keys()
            .map(|&n| (n, vec![]))
            .collect::<HashMap<_, _>>()
    } else {
        unreachable!();
    };

    for round in 0..1_000_000 {
        press_button(&mut module_map, &dests_map, |sender, _, pulse| {
            if history.contains_key(sender) && matches!(pulse, Pulse::High) {
                history.entry(sender).and_modify(|x| x.push(round));
            }
        });
        if history.values().all(|v| v.len() >= 2) {
            break;
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

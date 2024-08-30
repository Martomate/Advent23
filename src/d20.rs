use itertools::Itertools;
use queues::{IsQueue, Queue};
use std::{
    collections::HashMap,
    fmt::Display,
};

#[derive(Debug, PartialEq, Eq)]
enum ModuleState<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Signal>),
    Broadcast,
}

impl<'a> Display for ModuleState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ModuleState::*;

        match self {
            FlipFlop(on) => write!(f, "{}", if *on { '1' } else { '0' }),
            Conjunction(inputs) => {
                let inputs = inputs
                    .iter()
                    .sorted_by_key(|(&n, _)| n)
                    .map(|(_, &s)| format!("{}", if s == Signal::High { 'H' } else { 'L' }))
                    .join("");
                write!(f, "{}", inputs)
            }
            &Broadcast => write!(f, "_"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq)]
struct ModuleConfig<'a> {
    states: HashMap<&'a str, ModuleState<'a>>,
    destinations: HashMap<&'a str, Vec<&'a str>>,
}

struct Module<'a> {
    t: ModuleType,
    name: &'a str,
    dests: Vec<&'a str>,
}

impl<'a> From<(ModuleType, &'a str, Vec<&'a str>)> for Module<'a> {
    fn from((t, name, dests): (ModuleType, &'a str, Vec<&'a str>)) -> Self {
        Self { t, name, dests }
    }
}

fn parse_line(line: &str) -> Module {
    let (l, r) = line.split_once(" -> ").unwrap();

    let (name, module_type) = if let Some(name) = l.strip_prefix('%') {
        (name, ModuleType::FlipFlop)
    } else if let Some(name) = l.strip_prefix('&') {
        (name, ModuleType::Conjunction)
    } else if l == "broadcaster" {
        ("broadcaster", ModuleType::Broadcast)
    } else {
        panic!("unknown module type: {}", l);
    };

    Module {
        t: module_type,
        name,
        dests: r.split(", ").collect(),
    }
}

fn find_repeating_pattern(numbers: &[((u64, u64), Signal)]) -> Option<(u64, u64)> {
    for l in 1..=(numbers.len() / 2) {
        for start in 0..(l+2000) {
            if start + 10 * l > numbers.len() {
                break;
            }
            if numbers[start].1 != Signal::Low {
                continue; // we want the first event in the pattern to be (_, _, Low) which means the pattern starts out being High
            }
            let mut found = true;
            for i in start..(numbers.len() - l) {
                if numbers[i] != numbers[i + l] {
                    found = false;
                    break;
                }
            }
            if found {
                return Some((start as u64, l as u64));
            }
        }
    }
    None
}

fn parse_input(lines: Vec<&str>) -> ModuleConfig {
    ModuleConfig::from_lines(lines.into_iter().map(parse_line).collect())
}

impl<'a> ModuleConfig<'a> {
    fn from_lines<L: Into<Module<'a>>>(lines: Vec<L>) -> ModuleConfig<'a> {
        let mut states: HashMap<&str, ModuleState> = HashMap::new();
        let mut destinations: HashMap<&str, Vec<&str>> = HashMap::new();

        for line in lines {
            let Module { t, name, dests } = line.into();

            let state = match t {
                ModuleType::Broadcast => ModuleState::Broadcast,
                ModuleType::FlipFlop => ModuleState::FlipFlop(false),
                ModuleType::Conjunction => ModuleState::Conjunction(HashMap::new()),
            };
            //if !["br", "fk", "qk", "xt", "gh", "dj", "fj", "sl"].contains(&name) {
                states.insert(name, state);
                destinations.insert(name, dests);
            //}
        }

        // make sure output modules are part of the state space
        let mut outputs: Vec<&str> = Vec::new();
        for dests in destinations.values() {
            for &d in dests.iter() {
                if !states.contains_key(d) {
                    outputs.push(d);
                }
            }
        }
        for d in outputs {
            states.insert(d, ModuleState::Conjunction(HashMap::new()));
            destinations.insert(d, vec![]);
        }

        for (&from, dests) in destinations.iter() {
            for &d in dests.iter() {
                if let Some(ModuleState::Conjunction(inputs)) = &mut states.get_mut(d) {
                    inputs.insert(from, Signal::Low);
                }
            }
        }

        ModuleConfig {
            states,
            destinations,
        }
    }

    fn find_first_low(&mut self, target_name: &str) -> u64 {
        if target_name == "rx" {
            let mut q: Queue<(&str, Signal, &str)> = Queue::new();

            let mut outputs: HashMap<&str, Vec<_>> = HashMap::new();
            outputs.insert("km", vec![((0, 0), Signal::Low)]);
            outputs.insert("kz", vec![((0, 0), Signal::Low)]);
            outputs.insert("qs", vec![((0, 0), Signal::Low)]);
            outputs.insert("xj", vec![((0, 0), Signal::Low)]);

            for (group_input, group_output) in [("cr", "km"), ("fv", "kz"), ("tk", "qs"), ("rt", "xj")] {
                let mut presses: u64 = 0;

                let limit = 100000;
                for _ in 0..limit {
                    //q.add(("button", Signal::Low, "broadcaster")).unwrap();
                    q.add(("broadcaster", Signal::Low, group_input)).unwrap();
                    presses += 1;

                    let mut ticks: u64 = 0;

                    let mut backlog: usize = q.size();
                    while let Ok((from, signal, to)) = q.remove() {
                        backlog -= 1;

                        if from == group_output {
                            let &(last_t, last_s) = outputs.get(from).unwrap().last().unwrap();
                            if last_s != signal {
                                if last_t == (presses, ticks) {
                                    outputs.get_mut(from).unwrap().last_mut().unwrap().1 = signal;
                                } else {
                                    outputs.get_mut(from).unwrap().push(((presses, ticks), signal));
                                }
                            }
                        }
                        if let Some(state) = self.states.get_mut(to) {
                            let out = match state {
                                ModuleState::Broadcast => Some(signal),
                                ModuleState::FlipFlop(on) => match signal {
                                    Signal::High => None,
                                    Signal::Low => {
                                        *on = !*on;
                                        let out = if *on { Signal::High } else { Signal::Low };
                                        Some(out)
                                    }
                                },
                                ModuleState::Conjunction(last_inputs) => {
                                    last_inputs.insert(from, signal);
                                    let all_high = last_inputs.values().all(|&s| s == Signal::High);
                                    let out = if all_high { Signal::Low } else { Signal::High };
                                    Some(out)
                                }
                            };

                            if let Some(out) = out {
                                for &d in self.destinations.get(to).unwrap().iter() {
                                    q.add((to, out, d)).unwrap();
                                }
                            }
                        }
                        if backlog == 0 {
                            backlog = q.size();
                            ticks += 1;
                        }
                    }
                }
            }

            let mut patterns: HashMap<&str, _> = HashMap::new();

            for (label, outputs) in outputs.into_iter() {
                let mut nums = Vec::new();
                
                let mut prev_presses = 0;
                for ((p, t), s) in outputs {
                    let dp = p - prev_presses;

                    if let Some(((last_p, _), last_s)) = nums.last_mut() {
                        if *last_s == s {
                            *last_p += dp;
                        } else {
                            nums.push(((dp, t), s));
                        }
                    } else {
                        nums.push(((dp, t), s));
                    }
                    prev_presses = p;
                }

                if let Some((offset, length)) = find_repeating_pattern(&nums) {
                    let click_offset: u64 = nums.iter().take(offset as usize).map(|((p, _), _)| p).sum();
                    let click_length: u64 = nums.iter().dropping(offset as usize).take(length as usize).map(|((p, _), _)| p).sum();
                    let high_offset: u64 = nums.iter().dropping(offset as usize - 1).take(1).map(|((_, n), _)| n).sum();
                    let high_length: u64 = nums.iter().dropping(offset as usize).take(length as usize).filter(|(_, s)| *s == Signal::Low).map(|((p, n), _)| {
                        assert_eq!(*p, 0); // we do not support mulit-press high signals yet
                        *n - high_offset
                    }).sum();

                    patterns.insert(label, Pattern { click_offset, click_length, high_offset, high_length });
                }
            }

            let mut next_highs: HashMap<&str, Queue<(u64, u64)>> = HashMap::new();
            for (label, pattern) in patterns.iter() {
                next_highs.insert(label, Queue::new());

                for d in 0..pattern.high_length {
                    next_highs.get_mut(label).unwrap().add((pattern.click_offset, pattern.high_offset + d)).unwrap();
                }
            }

            let _n: u64 = patterns.values().map(|p| p.click_length).product();

            let mut possible_ticks: Vec<u64> = patterns.values().flat_map(|p| p.high_offset..(p.high_offset + p.high_length)).sorted().dedup().collect();
            possible_ticks.retain(|t| patterns.values().all(|p| (p.high_offset..(p.high_offset + p.high_length)).contains(t)));

            let mut first_solution = (u64::MAX, 0);

            for t in possible_ticks {
                let labels: Vec<&str> = patterns.keys().cloned().collect();

                let p = patterns.get(labels[0]).unwrap();
                let mut o1 = p.click_offset;
                let mut n1 = p.click_length;
                
                for &l in labels[1..].iter() {
                    let p2 = patterns.get(l).unwrap();
                    
                    let o2 = p2.click_offset;
                    let n2 = p2.click_length;

                    let mut a = o1;
                    loop {
                        a += n1;
                        if (a - o2) % n2 == 0 {
                            break;
                        }
                    }
                    
                    o1 = a;
                    n1 *= n2;
                }

                if o1 < first_solution.0 {
                    first_solution = (o1, t);
                }
            }

            first_solution.0
        } else {
            let mut q: Queue<(&str, Signal, &str)> = Queue::new();

            let mut presses: u64 = 0;

            let limit = 100000;
            for _ in 0..limit {
                q.add(("button", Signal::Low, "broadcaster")).unwrap();
                presses += 1;

                while let Ok((from, signal, to)) = q.remove() {
                    if to == target_name && signal == Signal::Low {
                        return presses;
                    }

                    if let Some(state) = self.states.get_mut(to) {
                        let out = match state {
                            ModuleState::Broadcast => Some(signal),
                            ModuleState::FlipFlop(on) => match signal {
                                Signal::High => None,
                                Signal::Low => {
                                    *on = !*on;
                                    let out = if *on { Signal::High } else { Signal::Low };
                                    Some(out)
                                }
                            },
                            ModuleState::Conjunction(last_inputs) => {
                                last_inputs.insert(from, signal);
                                let all_high = last_inputs.values().all(|&s| s == Signal::High);
                                let out = if all_high { Signal::Low } else { Signal::High };
                                Some(out)
                            }
                        };

                        if let Some(out) = out {
                            for &d in self.destinations.get(to).unwrap().iter() {
                                q.add((to, out, d)).unwrap();
                            }
                        }
                    }
                }
            }

            panic!("not found");
        }
    }

    fn simulate_button_press(&mut self) -> (u64, u64) {
        let mut low_count = 0;
        let mut high_count = 0;

        let mut q: Queue<(&str, Signal, &str)> = Queue::new();

        q.add(("button", Signal::Low, "broadcaster")).unwrap();

        while let Ok((from, signal, to)) = q.remove() {
            match signal {
                Signal::Low => low_count += 1,
                Signal::High => high_count += 1,
            };
            if let Some(state) = self.states.get_mut(to) {
                let out = match state {
                    ModuleState::Broadcast => Some(signal),
                    ModuleState::FlipFlop(on) => match signal {
                        Signal::High => None,
                        Signal::Low => {
                            *on = !*on;
                            let out = if *on { Signal::High } else { Signal::Low };
                            Some(out)
                        }
                    },
                    ModuleState::Conjunction(last_inputs) => {
                        last_inputs.insert(from, signal);
                        let all_high = last_inputs.values().all(|&s| s == Signal::High);
                        let out = if all_high { Signal::Low } else { Signal::High };
                        Some(out)
                    }
                };

                if let Some(out) = out {
                    for &d in self.destinations.get(to).unwrap().iter() {
                        q.add((to, out, d)).unwrap();
                    }
                }
            }
        }

        (low_count, high_count)
    }
}

#[derive(Debug)]
struct Pattern {
    click_offset: u64, // in clicks
    click_length: u64, // in clicks
    high_offset: u64, // in ticks
    high_length: u64, // in ticks
}

fn _make_state_str(modules: &ModuleConfig, compact: bool) -> String {
    modules
        .states
        .iter()
        .sorted_by_key(|(&n, _)| n)
        .map(|(_, state)| format!("{}", state))
        .join(if compact { "" } else { " " })
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let mut modules = parse_input(lines);

    if part1 {
        let mut total_low = 0;
        let mut total_high = 0;

        for _ in 0..1000 {
            let (low, high) = modules.simulate_button_press();
            total_low += low;
            total_high += high;
        }

        total_low * total_high
    } else {
        modules.find_first_low("rx")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_input() {
        use ModuleType::*;

        assert_eq!(
            parse_input(vec![
                "broadcaster -> a",
                "%a -> b",
                "&b -> c",
                "%c -> d",
                "%d -> output",
            ]),
            ModuleConfig::from_lines(vec![
                (Broadcast, "broadcaster", vec!["a"]),
                (FlipFlop, "a", vec!["b"]),
                (Conjunction, "b", vec!["c"]),
                (FlipFlop, "c", vec!["d"]),
                (FlipFlop, "d", vec!["output"]),
            ]),
        );
    }

    #[test]
    fn finding_first_low_signal() {
        use ModuleType::*;

        let mut modules = ModuleConfig::from_lines(vec![
            (Broadcast, "broadcaster", vec!["a"]),
            (FlipFlop, "a", vec!["b"]),
            (Conjunction, "b", vec!["c"]),
            (FlipFlop, "c", vec!["d"]),
            (FlipFlop, "d", vec!["output"]),
        ]);

        assert_eq!(modules.find_first_low("output"), 7);
    }
}

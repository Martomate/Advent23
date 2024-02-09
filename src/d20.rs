use queues::{IsQueue, Queue};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum ModuleState<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Signal>),
    Broadcast,
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
        Self {t, name, dests}
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

    Module { t: module_type, name, dests: r.split(", ").collect() }
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
            states.insert(name, state);
            destinations.insert(name, dests);
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
        let mut q: Queue<(&str, Signal, &str)> = Queue::new();

        // TODO: this ran for over 11 minutes, so I need to do something else
        let limit = 1000000000;
        for i in 0..limit {
            q.add(("button", Signal::Low, "broadcaster")).unwrap();

            while let Ok((from, signal, to)) = q.remove() {
                if signal == Signal::Low && to == target_name {
                    return i + 1;
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
        panic!("not found")
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

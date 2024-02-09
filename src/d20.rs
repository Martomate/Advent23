use queues::{IsQueue, Queue};
use std::collections::HashMap;

enum ModuleState<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Signal>),
    Broadcast,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Signal {
    Low,
    High,
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let mut states: HashMap<&str, ModuleState> = HashMap::new();
    let mut destinations: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines.iter() {
        let (l, r) = line.split_once(" -> ").unwrap();

        let (name, state) = if let Some(name) = l.strip_prefix('%') {
            (name, ModuleState::FlipFlop(false))
        } else if let Some(name) = l.strip_prefix('&') {
            (name, ModuleState::Conjunction(HashMap::new()))
        } else if l == "broadcaster" {
            ("broadcaster", ModuleState::Broadcast)
        } else {
            panic!("unknown module type: {}", l);
        };
        
        states.insert(name, state);
        destinations.insert(name, r.split(", ").collect());
    }

    for (&from, dests) in destinations.iter() {
        for &d in dests.iter() {
            if let Some(ModuleState::Conjunction(inputs)) = &mut states.get_mut(d) {
                inputs.insert(from, Signal::Low);
            }
        }
    }

    let mut q: Queue<(&str, Signal, &str)> = Queue::new();
    
    let mut low_count = 0;
    let mut high_count = 0;
    
    // TODO: this ran for over 11 minutes, so I need to do something else
    let limit = if part1 { 1000 } else { 1000000000 };
    for i in 0..limit {
        q.add(("button", Signal::Low, "broadcaster")).unwrap();

        while let Ok((from, signal, to)) = q.remove() {
            if !part1 && signal == Signal::Low && to == "rx" {
                return i + 1;
            }
            match signal {
                Signal::Low => low_count += 1,
                Signal::High => high_count += 1,
            };
            if let Some(state) = states.get_mut(to) {
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
                    for &d in destinations.get(to).unwrap().iter() {
                        q.add((to, out, d)).unwrap();
                    }
                }
            }
        }
    }

    if part1 {
        low_count * high_count
    } else {
        panic!("not found")
    }
}

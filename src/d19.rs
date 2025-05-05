use std::{collections::HashMap, ops::Range};

#[derive(Clone, Copy)]
enum Condition {
    GreaterThan(u8, u32),
    LessThan(u8, u32),
}

impl Condition {
    fn evaluate(&self, part: &Part) -> bool {
        match self {
            Condition::GreaterThan(name, v) => match *name {
                b'x' => part.x > *v,
                b'm' => part.m > *v,
                b'a' => part.a > *v,
                b's' => part.s > *v,
                _ => panic!(),
            },
            Condition::LessThan(name, v) => match *name {
                b'x' => part.x < *v,
                b'm' => part.m < *v,
                b'a' => part.a < *v,
                b's' => part.s < *v,
                _ => panic!(),
            },
        }
    }
}

fn parse_condition(s: &str) -> Condition {
    if let Some((name, num)) = s.split_once('<') {
        Condition::LessThan(name.as_bytes()[0], num.parse().unwrap())
    } else if let Some((name, num)) = s.split_once('>') {
        Condition::GreaterThan(name.as_bytes()[0], num.parse().unwrap())
    } else {
        panic!("unknown condition: {}", s);
    }
}

#[derive(Clone, Copy)]
enum Destination<'a> {
    Accepted,
    Rejected,
    Workflow(&'a str),
}

fn parse_destination(s: &str) -> Destination {
    match s {
        "A" => Destination::Accepted,
        "R" => Destination::Rejected,
        _ => Destination::Workflow(s),
    }
}

struct Rule<'a> {
    condition: Option<Condition>,
    destination: Destination<'a>,
}

impl<'a> Rule<'a> {
    fn evaluate(&'a self, part: &Part) -> Option<Destination<'a>> {
        if let Some(ref cond) = self.condition {
            if !cond.evaluate(part) {
                return None;
            }
        }
        Some(self.destination)
    }
}

fn parse_rule(s: &str) -> Rule {
    match s.split_once(':') {
        Some((left, right)) => Rule {
            condition: Some(parse_condition(left)),
            destination: parse_destination(right),
        },
        None => Rule {
            condition: None,
            destination: parse_destination(s),
        },
    }
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl Workflow<'_> {
    fn evaluate(&self, part: &Part) -> Destination {
        for rule in self.rules.iter() {
            if let Some(dest) = rule.evaluate(part) {
                return dest;
            }
        }
        panic!("the last rule should always match");
    }
}

fn parse_workflow(line: &str) -> Workflow {
    let (name, rest) = line.split_once('{').unwrap();
    let rest = rest.strip_suffix('}').unwrap();
    let rules = rest.split(',').map(parse_rule).collect();
    Workflow { name, rules }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone)]
struct PartRanges {
    x: Range<u16>,
    m: Range<u16>,
    a: Range<u16>,
    s: Range<u16>,
}

fn split_range_lt(r: &Range<u16>, d: u16) -> (Range<u16>, Range<u16>) {
    if d < r.start {
        (r.start..r.start, r.start..r.end)
    } else if d >= r.end {
        (r.start..r.end, r.start..r.start)
    } else {
        (r.start..d, d..r.end)
    }
}

impl PartRanges {
    fn split_lt(&self, name: u8, value: u32) -> (PartRanges, PartRanges) {
        match name {
            b'x' => {
                let (r_lt, r_ge) = split_range_lt(&self.x, value as u16);
                (
                    PartRanges {
                        x: r_lt,
                        ..self.clone()
                    },
                    PartRanges {
                        x: r_ge,
                        ..self.clone()
                    },
                )
            }
            b'm' => {
                let (r_lt, r_ge) = split_range_lt(&self.m, value as u16);
                (
                    PartRanges {
                        m: r_lt,
                        ..self.clone()
                    },
                    PartRanges {
                        m: r_ge,
                        ..self.clone()
                    },
                )
            }
            b'a' => {
                let (r_lt, r_ge) = split_range_lt(&self.a, value as u16);
                (
                    PartRanges {
                        a: r_lt,
                        ..self.clone()
                    },
                    PartRanges {
                        a: r_ge,
                        ..self.clone()
                    },
                )
            }
            b's' => {
                let (r_lt, r_ge) = split_range_lt(&self.s, value as u16);
                (
                    PartRanges {
                        s: r_lt,
                        ..self.clone()
                    },
                    PartRanges {
                        s: r_ge,
                        ..self.clone()
                    },
                )
            }
            _ => panic!(),
        }
    }
}

struct SortingSystem<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
}

impl<'a> SortingSystem<'a> {
    fn filter(&self, w_name: &'a str, part_ranges: PartRanges) -> Vec<PartRanges> {
        let start = self.workflows.get(w_name).unwrap();

        self.filter_rules(&start.rules, part_ranges)
    }

    fn filter_rules(&self, rules: &[Rule<'a>], ranges: PartRanges) -> Vec<PartRanges> {
        let mut results = Vec::new();

        if let Some((rule, next_rules)) = rules.split_first() {
            let dest = rule.destination;

            match rule.condition {
                Some(Condition::GreaterThan(name, v)) => {
                    let (ranges_le, ranges_gt) = ranges.split_lt(name, v + 1);

                    let res_gt = self.filter_destination(dest, ranges_gt);
                    let res_le = self.filter_rules(next_rules, ranges_le);

                    results.extend_from_slice(&res_gt);
                    results.extend_from_slice(&res_le);
                }
                Some(Condition::LessThan(name, v)) => {
                    let (ranges_lt, ranges_ge) = ranges.split_lt(name, v);

                    let res_lt = self.filter_destination(dest, ranges_lt);
                    let res_ge = self.filter_rules(next_rules, ranges_ge);

                    results.extend_from_slice(&res_lt);
                    results.extend_from_slice(&res_ge);
                }
                None => {
                    results.extend_from_slice(&self.filter_destination(dest, ranges));
                }
            };
        } else {
            panic!("ran out of rules")
        }

        results
    }

    fn filter_destination(&self, dest: Destination, ranges: PartRanges) -> Vec<PartRanges> {
        match dest {
            Destination::Accepted => vec![ranges],
            Destination::Rejected => vec![],
            Destination::Workflow(name) => self.filter(name, ranges),
        }
    }
}

fn parse_part(line: &str) -> Part {
    let line = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
    let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };
    for s in line.split(',') {
        let (name, value) = s.split_once('=').unwrap();
        let value = value.parse().unwrap();
        match name {
            "x" => part.x = value,
            "m" => part.m = value,
            "a" => part.a = value,
            "s" => part.s = value,
            _ => panic!("unknown category: {}", name),
        };
    }
    part
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let mut workflows = Vec::new();
    let mut parts = Vec::new();
    let mut workflows_done = false;

    for line in lines {
        if !workflows_done {
            if line.is_empty() {
                workflows_done = true;
            } else {
                workflows.push(parse_workflow(line));
            }
        } else {
            parts.push(parse_part(line));
        }
    }

    let workflows = workflows
        .into_iter()
        .map(|w| (w.name, w))
        .collect::<HashMap<_, _>>();

    let mut result = 0;

    if part1 {
        for p in parts {
            let mut w = workflows.get("in").unwrap();
            loop {
                println!("{}", w.name);
                match w.evaluate(&p) {
                    Destination::Accepted => {
                        result += (p.x + p.m + p.a + p.s) as u64;
                        break;
                    }
                    Destination::Rejected => {
                        break;
                    }
                    Destination::Workflow(name) => w = workflows.get(name).unwrap(),
                };
            }
        }
    } else {
        let sorting_system = SortingSystem { workflows };
        let res = sorting_system.filter(
            "in",
            PartRanges {
                x: 1..4001,
                m: 1..4001,
                a: 1..4001,
                s: 1..4001,
            },
        );
        for range in res {
            result += (range.x.len() * range.m.len() * range.a.len() * range.s.len()) as u64;
        }
    }

    result
}

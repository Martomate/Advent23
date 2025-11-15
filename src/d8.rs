use std::collections::HashMap;

use gcd::Gcd;

#[derive(Debug)]
struct Cycle {
    offsets: Vec<u64>,
    length: u64,
}

fn find_cycle(steps: &[char], mappings: &HashMap<&str, (&str, &str)>, start_node: &str) -> Cycle {
    let mut n: usize = 0;

    let mut cache: HashMap<(usize, &str), usize> = HashMap::new();
    let mut offsets: Vec<u64> = Vec::new();

    let mut current = start_node;
    loop {
        let s = n % steps.len();
        if let Some(&start) = cache.get(&(s, current)) {
            return Cycle {
                offsets,
                length: (n - start) as u64,
            };
        }
        cache.insert((s, current), n);
        if current.ends_with('Z') {
            offsets.push(n as u64);
        }

        let go_right = steps[s] == 'R';
        current = mappings
            .get(&current)
            .map(|(l, r)| if go_right { r } else { l })
            .unwrap();
        n += 1;
    }
}

/// Basic implementation of the Chinese Remainder Theorem for two equations
fn intersect(l: Cycle, r: Cycle) -> Cycle {
    let mut offsets = Vec::new();
    for la in l.offsets {
        for &ra in r.offsets.iter() {
            for n in 0..r.length {
                let x = la + n * l.length;
                if (x + r.length - ra).is_multiple_of(r.length) {
                    // found first match
                    offsets.push(x);
                    break;
                }
            }
        }
    }
    Cycle {
        offsets,
        length: l.length * r.length / l.length.gcd_binary(r.length),
    }
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let steps: Vec<char> = lines[0].chars().collect();

    let mut mappings: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines[2..].iter() {
        let (from, to) = line.split_once('=').unwrap();
        let (left, right) = to
            .trim()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(',')
            .unwrap();

        mappings.insert(from.trim(), (left.trim(), right.trim()));
    }

    if part1 {
        let mut current = "AAA";
        let mut num_steps = 0;
        while current != "ZZZ" {
            let &(l, r) = mappings.get(current).unwrap();
            if steps[num_steps % steps.len()] == 'R' {
                current = r;
            } else {
                current = l;
            }
            num_steps += 1;
        }
        num_steps as u64
    } else {
        mappings
            .keys()
            .copied()
            .filter(|s| s.ends_with('A'))
            .map(|node| find_cycle(&steps, &mappings, node))
            .reduce(intersect) // Chinese Remainder Theorem applied two cycles at a time
            .unwrap()
            .offsets
            .into_iter()
            .min()
            .unwrap()
    }
}

use std::collections::HashMap;

use itertools::{repeat_n, Itertools};

struct Matcher {
    pattern: Vec<char>,
    cache: HashMap<(usize, Vec<char>, usize), u64>,
}

impl Matcher {
    fn new(pattern: Vec<char>) -> Self {
        Self {
            pattern,
            cache: HashMap::new(),
        }
    }

    fn count_matches(&mut self, p_idx: usize, c: &[u32]) -> u64 {
        let mut count = 0;
        for i in p_idx..self.pattern.len() {
            let here = self.pattern[i];
            match here {
                '?' => {
                    let cache_key = (p_idx, self.pattern[p_idx..=i].to_vec(), c.len());

                    if let Some(value) = self.cache.get(&cache_key) {
                        return *value;
                    }

                    let mut total = 0;
                    if !c.is_empty() {
                        self.pattern[i] = '#';
                        total += self.count_matches(p_idx, c);
                    }
                    self.pattern[i] = '.';
                    total += self.count_matches(p_idx, c);
                    self.pattern[i] = '?';

                    self.cache.insert(cache_key, total);

                    return total;
                }
                '#' => {
                    count += 1;
                }
                '.' => {
                    if count != 0 {
                        if c.is_empty() || count != c[0] {
                            return 0;
                        } else {
                            return self.count_matches(i, &c[1..]);
                        }
                    }
                }
                _ => panic!("unknown character: {}", here),
            };
        }

        let is_match = match c {
            [] => count == 0,
            [e] => count == *e,
            _ => false,
        };

        if is_match {
            1
        } else {
            0
        }
    }
}

fn parse_line(line: &str, folds: usize) -> (Vec<char>, Vec<u32>) {
    let (s, c) = line.split_once(' ').unwrap();
    let s = repeat_n(s, folds).join("?");
    let c = repeat_n(c, folds).join(",");

    let pattern = s.chars().collect();
    let groups: Vec<u32> = c.split(',').map(|s| s.parse::<u32>().unwrap()).collect();

    (pattern, groups)
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let mut total = 0;
    for line in lines {
        let (s, c) = parse_line(line, if part1 { 1 } else { 5 });
        let res = Matcher::new(s).count_matches(0, &c);
        total += res;
    }
    total
}

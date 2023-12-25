use std::collections::HashMap;

struct Card {
    id: u32,
    winning: Vec<i32>,
    have: Vec<i32>,
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let (start, rest) = line.split_once(':').unwrap();
        let (left, right) = rest.split_once('|').unwrap();
        Card {
            id: start[5..].trim().parse().unwrap(),
            winning: parse_numbers(left),
            have: parse_numbers(right),
        }
    }
}

fn parse_numbers(line: &str) -> Vec<i32> {
    line.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn run(lines: Vec<&str>, part1: bool) -> u32 {
    if part1 {
        let mut total = 0;
        for line in lines {
            let card = Card::from(line);
            let mut s = 0;
            for n in card.have {
                if card.winning.contains(&n) {
                    if s == 0 {
                        s = 1;
                    } else {
                        s *= 2;
                    }
                }
            }
            total += s;
        }
        total
    } else {
        let mut total: u32 = 0;

        let mut cards = HashMap::new();
        for line in lines {
            let card = Card::from(line);
            cards.insert(card.id, card);
        }
        let mut ids: Vec<u32> = cards.keys().copied().collect();
        ids.sort();

        let mut count: HashMap<u32, u32> = ids.iter().map(|&id| (id, 1)).collect();

        for id in ids {
            let card = cards.remove(&id).unwrap();
            let mut m = 0;
            for n in card.have {
                if card.winning.contains(&n) {
                    m += 1;
                }
            }

            let c = count[&id];
            total += c;

            if m > 0 {
                for d in 1..=m {
                    let i = id + d;
                    count.insert(i, count[&i] + c);
                }
            }
        }
        total
    }
}

use std::{cmp::Ordering, collections::HashMap};

fn parse_card(c: char, use_jokers: bool) -> Result<u8, String> {
    match c {
        'A' => Ok(14),
        'K' => Ok(13),
        'Q' => Ok(12),
        'J' => Ok(if use_jokers { 1 } else { 11 }),
        'T' => Ok(10),
        '2'..='9' => Ok((c as u8) - b'0'),
        _ => Err("invalid card".to_string()),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cards([u8; 5]);

impl Cards {
    fn hand_type(&self) -> HandType {
        for i in 0..5 {
            if self.0[i] == 1 {
                let mut card = *self;
                let mut best = HandType::HighCard;
                for n in 2..15 {
                    card.0[i] = n;
                    let t = card.hand_type();
                    if t > best {
                        best = t;
                    }
                }
                return best;
            }
        }
        let mut counters: HashMap<u8, u8> = HashMap::new();
        for n in self.0 {
            counters.insert(n, counters.get(&n).unwrap_or(&0) + 1);
        }
        let c: Vec<u8> = counters.into_values().collect();

        if c.contains(&5) {
            HandType::FiveOfAKind
        } else if c.contains(&4) {
            HandType::FourOfAKind
        } else if c.contains(&3) {
            if c.contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if c.iter().filter(|&n| *n == 2).count() == 2 {
            HandType::TwoPairs
        } else if c.contains(&2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: Cards,
    bid: u64,
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type().cmp(&other.hand_type()).then_with(|| {
            let mut r = Ordering::Equal;
            for i in 0..5 {
                r = self.0[i].cmp(&other.0[i]);
                if r != Ordering::Equal {
                    break;
                }
            }
            r
        })
    }
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let mut hands = Vec::new();
    for line in lines {
        let mut cards = [0; 5];
        for (i, c) in line.chars().take(5).enumerate() {
            let card = parse_card(c, !part1).unwrap();
            cards[i] = card;
        }
        let bid = line[6..].parse::<u64>().unwrap();
        hands.push(Hand {
            cards: Cards(cards),
            bid,
        });
    }
    hands.sort_by_key(|h| h.cards);
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_hands() {
        assert_eq!(Cards([11, 11, 11, 11, 11]).hand_type(), HandType::FiveOfAKind);
        assert_eq!(Cards([11, 11, 11, 11, 12]).hand_type(), HandType::FourOfAKind);
        assert_eq!(Cards([11, 11, 11, 12, 12]).hand_type(), HandType::FullHouse);
        assert_eq!(Cards([11, 11, 11, 12, 13]).hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Cards([11, 11, 12, 12, 13]).hand_type(), HandType::TwoPairs);
        assert_eq!(Cards([11, 11, 12, 13, 14]).hand_type(), HandType::OnePair);
        assert_eq!(Cards([11, 12, 13, 14, 15]).hand_type(), HandType::HighCard);
    }

    #[test]
    fn type_hands_with_jokes() {
        assert_eq!(Cards([1, 11, 11, 11, 11]).hand_type(), HandType::FiveOfAKind);
        assert_eq!(Cards([1, 11, 11, 11, 12]).hand_type(), HandType::FourOfAKind);
        assert_eq!(Cards([1, 11, 11, 12, 12]).hand_type(), HandType::FullHouse);
        assert_eq!(Cards([1, 11, 11, 12, 13]).hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Cards([1, 11, 12, 12, 13]).hand_type(), HandType::ThreeOfAKind); // !!
        assert_eq!(Cards([1, 11, 12, 13, 14]).hand_type(), HandType::OnePair);
        assert_eq!(Cards([1, 12, 13, 14, 15]).hand_type(), HandType::OnePair); // !!
    }

    #[test]
    fn sort_hands() {
        let mut cards = [
            Cards([11, 11, 11, 11, 11]),
            Cards([11, 11, 11, 11, 12]),
            Cards([11, 11, 11, 12, 12]),
            Cards([11, 11, 11, 12, 13]),
            Cards([11, 11, 12, 12, 13]),
            Cards([11, 11, 12, 13, 14]),
            Cards([11, 12, 13, 14, 10]),
        ];
        cards.sort();
        assert_eq!(cards, [
            Cards([11, 12, 13, 14, 10]),
            Cards([11, 11, 12, 13, 14]),
            Cards([11, 11, 12, 12, 13]),
            Cards([11, 11, 11, 12, 13]),
            Cards([11, 11, 11, 12, 12]),
            Cards([11, 11, 11, 11, 12]),
            Cards([11, 11, 11, 11, 11]),
        ]);
    }
}

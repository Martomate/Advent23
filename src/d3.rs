use std::collections::HashMap;

fn parse_symbols(lines: &[&str]) -> HashMap<(usize, usize), char> {
    let mut symbols = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if !ch.is_ascii_digit() && ch != '.' {
                symbols.insert((x, y), ch);
            }
        }
    }
    symbols
}

#[derive(Debug, PartialEq, Eq)]
struct Number {
    x: usize,
    y: usize,
    digits: usize,
    value: u32,
}

fn parse_numbers(lines: &[&str]) -> Vec<Number> {
    let mut numbers = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut start_x = 0;
        let mut number: u32 = 0;
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if number == 0 {
                    start_x = x;
                }
                number *= 10;
                number += (ch as u32) - ('0' as u32);
            } else if number > 0 {
                numbers.push(Number {
                    x: start_x,
                    y,
                    digits: x - start_x,
                    value: number,
                });
                number = 0;
            }
        }
        if number > 0 {
            numbers.push(Number {
                x: start_x,
                y,
                digits: line.len() - start_x,
                value: number,
            });
        }
    }
    numbers
}

pub fn run(lines: Vec<&str>, part1: bool) -> u32 {
    let symbols = parse_symbols(&lines);
    let numbers = parse_numbers(&lines);

    if part1 {
        let mut total = 0;
        for num in numbers {
            let mut found = false;
            for dy in -1..=1 {
                for dx in -1..=num.digits as isize {
                    let nx = num.x as isize + dx;
                    let ny = num.y as isize + dy;
                    if nx >= 0 && ny >= 0 && symbols.contains_key(&(nx as usize, ny as usize)) {
                        total += num.value;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
        total
    } else {
        let mut total: u32 = 0;
        let mut gear_number_counts: HashMap<(usize, usize), u32> = HashMap::new();
        let mut gear_ratios: HashMap<(usize, usize), u32> = HashMap::new();
        for num in numbers {
            for dy in -1..=1 {
                for dx in -1..=num.digits as isize {
                    let nx = num.x as isize + dx;
                    let ny = num.y as isize + dy;
                    let coords = (nx as usize, ny as usize);
                    if nx >= 0 && ny >= 0 && symbols.get(&coords) == Some(&'*') {
                        gear_number_counts
                            .insert(coords, gear_number_counts.get(&coords).unwrap_or(&0) + 1);
                        gear_ratios
                            .insert(coords, gear_ratios.get(&coords).unwrap_or(&1) * num.value);
                    }
                }
            }
        }
        for ((x, y), count) in gear_number_counts {
            if count == 2 {
                total += gear_ratios.get(&(x, y)).unwrap();
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use crate::d3::Number;

    use super::parse_numbers;

    #[test]
    fn parse_numbers_simple() {
        assert_eq!(
            parse_numbers(&["123"]),
            vec![Number {
                x: 0,
                y: 0,
                digits: 3,
                value: 123
            }]
        );
    }

    #[test]
    fn parse_numbers_with_dots() {
        assert_eq!(
            parse_numbers(&["..123.."]),
            vec![Number {
                x: 2,
                y: 0,
                digits: 3,
                value: 123
            }]
        );
    }
}

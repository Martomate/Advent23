#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn possible(&self, max: CubeSet) -> bool {
        for set in self.sets.iter() {
            if set.red > max.red || set.green > max.green || set.blue > max.blue {
                return false;
            }
        }
        true
    }
}

fn parse_game(line: &str) -> Game {
    let (left, right) = line.split_once(':').unwrap();
    let id_str = left.split_once(' ').unwrap().1;
    let id = id_str.parse::<u32>().unwrap();

    let mut sets = Vec::new();
    for set in right.split(';') {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for part in set.split(',') {
            let (c, t) = part.trim().split_once(' ').unwrap();
            let v = c.trim().parse::<u32>().unwrap();
            match t.trim() {
                "red" => red = v,
                "green" => green = v,
                "blue" => blue = v,
                _ => panic!("what?"),
            }
        }
        sets.push(CubeSet::new(red, green, blue));
    }
    Game { id, sets }
}

pub fn run(lines: Vec<&str>, part1: bool) -> u32 {
    if part1 {
        lines
            .into_iter()
            .map(parse_game)
            .filter(|game| game.possible(CubeSet::new(12, 13, 14)))
            .map(|game| game.id)
            .sum()
    } else {
        lines
            .into_iter()
            .map(parse_game)
            .map(|game| {
                let mut req = CubeSet::new(0, 0, 0);
                for set in game.sets.iter() {
                    if set.red > req.red {
                        req.red = set.red;
                    }
                    if set.green > req.green {
                        req.green = set.green;
                    }
                    if set.blue > req.blue {
                        req.blue = set.blue;
                    }
                }
                req.red * req.green * req.blue
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_works() {
        let game = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let expected = Game {
            id: 1,
            sets: vec![
                CubeSet::new(4, 0, 3),
                CubeSet::new(1, 2, 6),
                CubeSet::new(0, 2, 0),
            ],
        };
        assert_eq!(game, expected);
    }
}

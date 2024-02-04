use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    fn forward(self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::Up => (x, y - 1),
            Dir::Left => (x - 1, y),
            Dir::Down => (x, y + 1),
            Dir::Right => (x + 1, y),
        }
    }

    fn left(self) -> Dir {
        use Dir::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn right(self) -> Dir {
        use Dir::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug)]
struct PlanStep {
    dir: Dir,
    steps: u32,
}

impl PlanStep {
    fn from_line(line: &str, use_color: bool) -> Result<Self, String> {
        let (dir, steps, color) = line
            .split(' ')
            .collect_tuple()
            .ok_or_else(|| "wrong number of spaces".to_string())?;

        if use_color {
            // this is a bit overkill, but good practice in parsing
            let color = color
                .strip_prefix('(')
                .ok_or_else(|| "missing '(' before color".to_string())?
                .strip_prefix('#')
                .ok_or_else(|| "missing '#' before color".to_string())?
                .strip_suffix(')')
                .ok_or_else(|| "missing ')' after color".to_string())?;

            let dir = color.chars().nth(5).unwrap();
            let dir = match dir {
                '0' => Dir::Right,
                '1' => Dir::Down,
                '2' => Dir::Left,
                '3' => Dir::Up,
                _ => return Err(format!("unknown hex direction: {}", dir)),
            };

            let steps = u32::from_str_radix(&color[..5], 16)
                .map_err(|e| format!("invalid hex number: {}", e))?;

            Ok(PlanStep { dir, steps })
        } else {
            let dir = match dir {
                "U" => Dir::Up,
                "L" => Dir::Left,
                "D" => Dir::Down,
                "R" => Dir::Right,
                _ => return Err(format!("unknown direction: {}", dir)),
            };

            let steps = steps
                .parse::<u32>()
                .map_err(|e| format!("failed to parse steps as int: {}", e))?;

            Ok(PlanStep { dir, steps })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Coloring {
    Left,
    Right,
    Mid,
}

fn execute_plan(plan: Vec<PlanStep>) -> u64 {
    let mut coloring: HashMap<(i32, i32), Coloring> = HashMap::new();

    let mut here = (0, 0);

    for s in plan {
        coloring.insert(here, Coloring::Mid);
        coloring
            .entry(s.dir.left().forward(here))
            .or_insert(Coloring::Left);
        coloring
            .entry(s.dir.right().forward(here))
            .or_insert(Coloring::Right);
        if s.dir == Dir::Left || s.dir == Dir::Right {
            for _ in 0..s.steps {
                here = s.dir.forward(here);
                coloring.insert(here, Coloring::Mid);
            }
        } else {
        for _ in 0..s.steps {
            here = s.dir.forward(here);
            coloring.insert(here, Coloring::Mid);
            coloring
                .entry(s.dir.left().forward(here))
                .or_insert(Coloring::Left);
            coloring
                .entry(s.dir.right().forward(here))
                .or_insert(Coloring::Right);
        }}
    }

    let mut holes: Vec<(i32, i32)> = coloring
        .iter()
        .filter_map(|(k, c)| (*c == Coloring::Mid).then_some(k))
        .cloned()
        .collect();

    holes.sort_by(|(x1, y1), (x2, y2)| y1.cmp(y2).then_with(|| x1.cmp(x2)));

    // sorted x-coordinates for each row
    let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();
    for &(x, y) in holes.iter() {
        rows.entry(y).or_default().push(x);
    }

    let mut total = 0;

    for (y, xs) in rows {
        let mut row_total = xs.len() as u64; // include the edges

        let mut inside_coloring = Coloring::Mid; // replaced below
        let mut prev_x = None;

        for &x in xs.iter() {
            if inside_coloring == Coloring::Mid {
                inside_coloring = *coloring.get(&Dir::Right.forward((x, y))).unwrap();
            }
            if let Some(prev_x) = prev_x {
                let d = x - prev_x;
                if d > 1 {
                    let c = *coloring.get(&Dir::Left.forward((x, y))).unwrap();
                    if c == inside_coloring {
                        row_total += (d - 1) as u64; // include the space between the edges
                    }
                }
            } else {
                let outside_coloring = *coloring.get(&Dir::Left.forward((x, y))).unwrap();
                inside_coloring = match outside_coloring {
                    Coloring::Left => Coloring::Right,
                    Coloring::Right => Coloring::Left,
                    Coloring::Mid => unreachable!(),
                };
            }
            prev_x = Some(x);
        }

        total += row_total;
    }

    total
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let mut plan = Vec::new();

    for line in lines {
        plan.push(PlanStep::from_line(line, !part1).unwrap());
    }

    execute_plan(plan)
}

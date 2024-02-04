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
    fn forward(self, (x, y): (isize, isize), steps: isize) -> (isize, isize) {
        match self {
            Dir::Up => (x, y - steps),
            Dir::Left => (x - steps, y),
            Dir::Down => (x, y + steps),
            Dir::Right => (x + steps, y),
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

fn gridify(xs: Vec<i32>) -> (Vec<i32>, Vec<u32>) {
    let xs = xs.iter().cloned().unique().sorted().collect::<Vec<i32>>();
    
    let mut grid = Vec::new();
    let mut mult = Vec::new();

    grid.push(xs[0]);
    mult.push(1);

    let mut prev_x = xs[0];
    for &x in xs[1..].iter() {
        if x > prev_x + 1 {
            grid.push(prev_x+1);
            mult.push((x - prev_x - 1) as u32);
        }
        grid.push(x);
        mult.push(1);
        
        prev_x = x;
    }

    (grid, mult)
}

fn execute_plan(plan: Vec<PlanStep>) -> u64 {
    let mut xs = Vec::new();
    let mut ys= Vec::new();
    let mut here = (0, 0);
    for s in plan.iter() {
        let there = s.dir.forward(here, s.steps as isize);
        xs.push(there.0 as i32);
        ys.push(there.1 as i32);
        here = there;
    }

    let (x_grid, x_mult) = gridify(xs);
    let (y_grid, y_mult) = gridify(ys);

    let mut coords: HashMap<(i32, i32), (isize, isize)> = HashMap::new();
    let mut mult: HashMap<(isize, isize), u64> = HashMap::new();
    for i in 0..x_grid.len() {
        for j in 0..y_grid.len() {
            coords.insert((x_grid[i], y_grid[j]), (i as isize, j as isize));
            mult.insert((i as isize, j as isize), x_mult[i] as u64 * y_mult[j] as u64);
        }
    }

    let mut coloring: HashMap<(isize, isize), Coloring> = HashMap::new();

    let mut here = *coords.get(&(0, 0)).unwrap();

    for s in plan {
        coloring.insert(here, Coloring::Mid);
        coloring
            .entry(s.dir.left().forward(here, 1))
            .or_insert(Coloring::Left);
        coloring
            .entry(s.dir.right().forward(here, 1))
            .or_insert(Coloring::Right);
        
        let here_real = (x_grid[here.0 as usize], y_grid[here.1 as usize]);
        let (dx, dy) = s.dir.forward((0, 0), 1);
        let dest_real = (here_real.0 + dx as i32 * s.steps as i32, here_real.1 + dy as i32 * s.steps as i32);
        let dest = *coords.get(&dest_real).unwrap();
        let steps = dest.0.abs_diff(here.0) + dest.1.abs_diff(here.1);

        if s.dir == Dir::Left || s.dir == Dir::Right {
            for _ in 0..steps {
                here = s.dir.forward(here, 1);
                coloring.insert(here, Coloring::Mid);
            }
        } else {
            for _ in 0..steps {
                here = s.dir.forward(here, 1);
                coloring.insert(here, Coloring::Mid);
                coloring
                    .entry(s.dir.left().forward(here, 1))
                    .or_insert(Coloring::Left);
                coloring
                    .entry(s.dir.right().forward(here, 1))
                    .or_insert(Coloring::Right);
            }
        }
    }

    let mut holes: Vec<(isize, isize)> = coloring
        .iter()
        .filter_map(|(k, c)| (*c == Coloring::Mid).then_some(k))
        .cloned()
        .collect();

    holes.sort_by(|(x1, y1), (x2, y2)| y1.cmp(y2).then_with(|| x1.cmp(x2)));

    // sorted x-coordinates for each row
    let mut rows: HashMap<isize, Vec<isize>> = HashMap::new();
    for &(x, y) in holes.iter() {
        rows.entry(y).or_default().push(x);
    }

    let mut total = 0;

    for (y, xs) in rows {
        let mut row_total = 0;

        let mut inside_coloring = Coloring::Mid; // replaced below
        let mut prev_x = None;

        for &x in xs.iter() {
            row_total += *mult.get(&(x, y)).unwrap();

            if inside_coloring == Coloring::Mid {
                inside_coloring = *coloring.get(&Dir::Right.forward((x, y), 1)).unwrap();
            }
            if let Some(prev_x) = prev_x {
                let d = x - prev_x;
                if d > 1 {
                    let c = *coloring.get(&Dir::Left.forward((x, y), 1)).unwrap();
                    if c == inside_coloring {
                        for xx in (prev_x+1)..x {
                            row_total += *mult.get(&(xx, y)).unwrap();
                        }
                    }
                }
            } else {
                let outside_coloring = *coloring.get(&Dir::Left.forward((x, y), 1)).unwrap();
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

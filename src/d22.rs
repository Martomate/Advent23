use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn parse_str(s: &str) -> Self {
        let nums: Vec<i64> = s.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

        let [x, y, z] = nums[..3] else {
            panic!("expected 3 integers")
        };

        Self { x, y, z }
    }

    fn add_x(&self, d: i64) -> Self {
        let mut res = *self;
        res.x += d;
        res
    }

    fn add_y(&self, d: i64) -> Self {
        let mut res = *self;
        res.y += d;
        res
    }

    fn add_z(&self, d: i64) -> Self {
        let mut res = *self;
        res.z += d;
        res
    }
}

#[derive(Debug, Clone)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
struct Brick {
    start: Point,
    axis: Axis,
    dist: u64,
}

fn min_by<K: Ord>(from: Point, to: Point, key: impl Fn(&Point) -> K) -> Point {
    if key(&from) <= key(&to) {
        from
    } else {
        to
    }
}

impl Brick {
    fn new(start: Point, axis: Axis, dist: u64) -> Self {
        Self { start, axis, dist }
    }

    fn points(&self) -> Vec<Point> {
        match self.axis {
            Axis::X => (0..=self.dist)
                .map(|d| self.start.add_x(d as i64))
                .collect(),
            Axis::Y => (0..=self.dist)
                .map(|d| self.start.add_y(d as i64))
                .collect(),
            Axis::Z => (0..=self.dist)
                .map(|d| self.start.add_z(d as i64))
                .collect(),
        }
    }

    fn will_fall(&self, spots: &mut HashSet<Point>) -> bool {
        for p in self.points() {
            spots.remove(&p);
        }
        let res = self.start.z > 1
            && !self
                .points()
                .into_iter()
                .any(|p| spots.contains(&p.add_z(-1)));
        for p in self.points() {
            spots.insert(p);
        }
        res
    }

    fn is_above(&self, other: &Brick) -> bool {
        for p1 in self.points() {
            for p2 in other.points() {
                if p1 == p2.add_z(1) {
                    return true;
                }
            }
        }
        false
    }

    fn from_points(from: Point, to: Point) -> Self {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let dz = to.z - from.z;

        match (dx, dy, dz) {
            (0, 0, 0) => Brick::new(from, Axis::X, 0),
            (dx, 0, 0) => Brick::new(min_by(from, to, |p| p.x), Axis::X, dx.unsigned_abs()),
            (0, dy, 0) => Brick::new(min_by(from, to, |p| p.y), Axis::Y, dy.unsigned_abs()),
            (0, 0, dz) => Brick::new(min_by(from, to, |p| p.z), Axis::Z, dz.unsigned_abs()),
            _ => panic!("only one axis may change"),
        }
    }
}

pub fn run(lines: Vec<&str>, chain_reaction: bool) -> u64 {
    let mut bricks = Vec::new();

    for line in lines {
        let (from_str, to_str) = line.split_once('~').unwrap();
        let from = Point::parse_str(from_str);
        let to = Point::parse_str(to_str);

        bricks.push(Brick::from_points(from, to));
    }

    let mut spots: HashSet<Point> = HashSet::new();
    for b in &bricks {
        for p in b.points() {
            spots.insert(p);
        }
    }

    loop {
        let mut moved = false;
        for b in &mut bricks {
            if b.start.z <= 1 {
                // already on the ground
                continue;
            }
            for p in b.points() {
                spots.remove(&p);
            }

            let mut can_move = true;
            for p in b.points() {
                if spots.contains(&p.add_z(-1)) {
                    can_move = false;
                    break;
                }
            }

            if can_move {
                for p in b.points() {
                    spots.insert(p.add_z(-1));
                }
                b.start = b.start.add_z(-1);
                moved = true;
            } else {
                for p in b.points() {
                    spots.insert(p);
                }
            }
        }
        if !moved {
            break;
        }
    }

    let bricks = bricks; // remove mut

    if let Some(b) = bricks.iter().find(|b| b.will_fall(&mut spots)) {
        panic!("brick can still fall: {:?}", b);
    }

    let mut dependents: Vec<Vec<usize>> = (0..bricks.len()).map(|_| Vec::new()).collect();
    let mut dependencies: Vec<Vec<usize>> = (0..bricks.len()).map(|_| Vec::new()).collect();

    for (i1, b1) in bricks.iter().enumerate() {
        for (i2, b2) in bricks.iter().enumerate() {
            if i1 != i2 && b2.is_above(b1) {
                dependents[i1].push(i2);
                dependencies[i2].push(i1);
            }
        }
    }

    for i in 0..bricks.len() {
        if bricks[i].start.z == 1 {
            dependencies[i].push(bricks.len());
        }
    }

    let dependents = dependents;
    let dependencies = dependencies;

    let mut num: Vec<u64> = std::iter::repeat_n(0, bricks.len()).collect();

    for (s_idx, _) in bricks.iter().enumerate() {
        let mut dependencies = dependencies.clone();

        dependencies[s_idx].clear(); // make this brick removable

        let mut removed: Vec<bool> = std::iter::repeat_n(false, bricks.len()).collect();

        loop {
            let mut added = 0;

            for (i, _) in bricks.iter().enumerate() {
                if !removed[i] && dependencies[i].is_empty() {
                    for &j in dependents[i].iter() {
                        let index = dependencies[j].iter().position(|&d| d == i).unwrap();
                        dependencies[j].remove(index);
                    }
                    removed[i] = true;
                    added += 1;
                }
            }
            if added == 0 {
                break;
            }
            num[s_idx] += added;
        }

        num[s_idx] -= 1; // don't count the brick that was removed
    }

    if chain_reaction {
        num.into_iter().sum()
    } else {
        num.into_iter().filter(|&n| n == 0).count() as u64
    }
}

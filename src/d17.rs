use std::{
    cmp::Reverse,
    collections::HashSet,
};

use priority_queue::PriorityQueue;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn rotated_cw(self) -> Self {
        use Direction::*;

        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn rotated_ccw(self) -> Self {
        use Direction::*;

        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
    forwards_left: u8,
}

impl State {
    fn new(x: usize, y: usize, dir: Direction, forwards_left: u8) -> State {
        State {
            x,
            y,
            dir,
            forwards_left,
        }
    }

    fn moved_forward(&self, w: usize, h: usize) -> Option<State> {
        let State {
            x,
            y,
            dir,
            forwards_left: f,
        } = *self;

        if f == 0 {
            return None;
        }

        #[allow(clippy::unnecessary_lazy_evaluations)]
        let (nx, ny) = match dir {
            Direction::Up => (y > 0).then(|| (x, y - 1)),
            Direction::Left => (x > 0).then(|| (x - 1, y)),
            Direction::Down => (y < h - 1).then(|| (x, y + 1)),
            Direction::Right => (x < w - 1).then(|| (x + 1, y)),
        }?;

        Some(State::new(nx, ny, dir, f - 1))
    }
}

struct Heatmap {
    rows: Vec<Vec<u8>>,
}

impl Heatmap {
    fn find_best_path(&self, min_forward: u8, max_forward: u8) -> u32 {
        let width = self.rows[0].len();
        let height = self.rows.len();

        let mut pq: PriorityQueue<(State, u32), Reverse<u32>> = PriorityQueue::new();
        let mut seen: HashSet<State> = HashSet::new();

        pq.push((State::new(0, 0, Direction::Right, max_forward), 0), Reverse(0));
        pq.push((State::new(0, 0, Direction::Down, max_forward), 0), Reverse(0));

        while let Some(((state, heat), _)) = pq.pop() {
            if !seen.insert(state.clone()) {
                continue;
            }

            let State {
                x,
                y,
                dir,
                forwards_left,
            } = state;

            if x == width - 1 && y == height - 1 {
                if max_forward - forwards_left < min_forward {
                    continue;
                }
                return heat;
            }

            for (dir, reset_f) in [
                (dir, false),
                (dir.rotated_cw(), true),
                (dir.rotated_ccw(), true),
            ] {
                if reset_f && max_forward - forwards_left < min_forward {
                    continue;
                }
                let s = State {
                    dir,
                    forwards_left: if reset_f { max_forward } else { forwards_left },
                    ..state
                };

                if let Some(n) = s.moved_forward(width, height) {
                    let h = heat + self.rows[n.y][n.x] as u32;
                    pq.push((n, h), Reverse(h));
                }
            }
        }

        panic!("destination was not found")
    }
}

fn parse_heatmap(lines: Vec<&str>) -> Heatmap {
    let mut rows = Vec::new();

    for line in lines {
        rows.push(line.chars().map(|ch| ch as u8 - b'0').collect());
    }

    Heatmap { rows }
}

pub fn run(lines: Vec<&str>, part1: bool) -> u32 {
    let heatmap = parse_heatmap(lines);
    if part1 {
        heatmap.find_best_path(1, 3)
    } else {
        heatmap.find_best_path(4, 10)
    }
}

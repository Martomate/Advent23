use std::collections::HashSet;

use itertools::Itertools;
use queues::{IsQueue, Queue};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    BackMirror,
    ForwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

enum TileEffect {
    Just(Direction),
    Both(Direction, Direction),
}

impl Tile {
    fn apply(self, dir: Direction) -> TileEffect {
        use Direction::*;
        use Tile::*;
        use TileEffect::*;

        match self {
            Empty => Just(dir),
            BackMirror => Just(match dir {
                Up => Left,
                Left => Up,
                Down => Right,
                Right => Down,
            }),
            ForwardMirror => Just(match dir {
                Up => Right,
                Right => Up,
                Down => Left,
                Left => Down,
            }),
            VerticalSplitter => match dir {
                Up | Down => Just(dir),
                Left | Right => Both(Up, Down),
            },
            HorizontalSplitter => match dir {
                Left | Right => Just(dir),
                Up | Down => Both(Left, Right),
            },
        }
    }
}

struct Grid {
    rows: Vec<Vec<Tile>>,
}

impl Grid {
    fn tile_at(&self, x: usize, y: usize) -> Tile {
        self.rows[y][x]
    }

    #[allow(clippy::unnecessary_lazy_evaluations)] // remove when Rust 1.76 has been released
    fn coords_in_front_of(&self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::Up => (y != 0).then(|| (x, y - 1)),
            Direction::Left => (x != 0).then(|| (x - 1, y)),
            Direction::Down => (y < self.rows.len() - 1).then_some((x, y + 1)),
            Direction::Right => (x < self.rows[0].len() - 1).then_some((x + 1, y)),
        }
    }

    fn simulate_beam(&self, start: (usize, usize, Direction)) -> u32 {
        let mut q: Queue<(usize, usize, Direction)> = Queue::new();
        q.add(start).unwrap();

        let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();

        while let Ok(state) = q.remove() {
            if !visited.insert(state) {
                continue;
            }

            let (x, y, dir) = state;
            let tile = self.tile_at(x, y);
            match tile.apply(dir) {
                TileEffect::Just(d) => {
                    if let Some((nx, ny)) = self.coords_in_front_of(x, y, d) {
                        q.add((nx, ny, d)).unwrap();
                    }
                }
                TileEffect::Both(d1, d2) => {
                    for d in [d1, d2] {
                        if let Some((nx, ny)) = self.coords_in_front_of(x, y, d) {
                            q.add((nx, ny, d)).unwrap();
                        }
                    }
                }
            }
        }

        visited.iter().map(|(x, y, _)| (x, y)).unique().count() as u32
    }
}

fn parse_grid(lines: Vec<&str>) -> Grid {
    let mut rows = Vec::new();

    for line in lines {
        rows.push(
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::Empty,
                    '\\' => Tile::BackMirror,
                    '/' => Tile::ForwardMirror,
                    '|' => Tile::VerticalSplitter,
                    '-' => Tile::HorizontalSplitter,
                    _ => panic!("invalid tile: {}", ch),
                })
                .collect(),
        );
    }

    Grid { rows }
}

pub fn run(lines: Vec<&str>, part1: bool) -> u32 {
    let grid = parse_grid(lines);
    if part1 {
        grid.simulate_beam((0, 0, Direction::Right))
    } else {
        let w = grid.rows[0].len();
        let h = grid.rows.len();

        let mut best = 0;

        for x in 0..w {
            let res = grid.simulate_beam((x, 0, Direction::Down));
            if res > best {
                best = res;
            }
            let res = grid.simulate_beam((x, h - 1, Direction::Up));
            if res > best {
                best = res;
            }
        }

        for y in 0..h {
            let res = grid.simulate_beam((0, y, Direction::Right));
            if res > best {
                best = res;
            }
            let res = grid.simulate_beam((w - 1, y, Direction::Left));
            if res > best {
                best = res;
            }
        }

        best
    }
}

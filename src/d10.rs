use std::fmt::{Display, Write};

use queues::{IsQueue, Queue};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn opposite(self) -> Self {
        use Dir::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    fn rotated_cw(self) -> Self {
        use Dir::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    fn rotated_ccw(self) -> Self {
        self.rotated_cw().opposite()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Pipe {
    fn connects_to(&self, dir: Dir) -> bool {
        let (d1, d2) = self.dirs();
        d1 == dir || d2 == dir
    }

    fn dirs(&self) -> (Dir, Dir) {
        use Dir::*;

        match self {
            Pipe::Vertical => (Up, Down),
            Pipe::Horizontal => (Left, Right),
            Pipe::UpLeft => (Up, Left),
            Pipe::UpRight => (Up, Right),
            Pipe::DownLeft => (Down, Left),
            Pipe::DownRight => (Down, Right),
        }
    }
}

impl From<Pipe> for char {
    fn from(pipe: Pipe) -> Self {
        match pipe {
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::UpLeft => 'J',
            Pipe::UpRight => 'L',
            Pipe::DownLeft => '7',
            Pipe::DownRight => 'F',
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'J' => Ok(Pipe::UpLeft),
            'L' => Ok(Pipe::UpRight),
            '7' => Ok(Pipe::DownLeft),
            'F' => Ok(Pipe::DownRight),
            _ => Err(format!("invalid pipe character: {}", ch)),
        }
    }
}

impl TryFrom<(Dir, Dir)> for Pipe {
    type Error = String;

    fn try_from(dirs: (Dir, Dir)) -> Result<Self, Self::Error> {
        use Dir::*;

        match dirs {
            (Up, Down) | (Down, Up) => Ok(Pipe::Vertical),
            (Left, Right) | (Right, Left) => Ok(Pipe::Horizontal),
            (Up, Left) | (Left, Up) => Ok(Pipe::UpLeft),
            (Up, Right) | (Right, Up) => Ok(Pipe::UpRight),
            (Down, Left) | (Left, Down) => Ok(Pipe::DownLeft),
            (Down, Right) | (Right, Down) => Ok(Pipe::DownRight),
            _ => Err(format!("invalid pipe directions: {:?}", dirs)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Coloring {
    Path,
    Inside,
    Outside,
}

struct Spot {
    pipe: Option<Pipe>,
    coloring: Option<Coloring>,
}

struct Grid {
    spots: Vec<Vec<Spot>>,
    // TODO: store spot category here (Path, Left, Right) which will later be used to flood fill and finally decide which of Left and Right is Inside and Outside
    //       Alternatively just make a Cell struct with an Option<Pipe> and an Option<Type>
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.spots.iter() {
            for spot in row.iter() {
                let ch;
                if let Some(coloring) = spot.coloring {
                    ch = match coloring {
                        Coloring::Path => '#',
                        Coloring::Inside => 'I',
                        Coloring::Outside => 'O',
                    };
                } else {
                    ch = match spot.pipe {
                        Some(pipe) => pipe.into(),
                        None => '.',
                    };
                }
                f.write_char(ch)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    fn neighbor_coords(&self, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::Up => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Dir::Down => {
                if y < self.spots.len() - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Dir::Left => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Dir::Right => {
                if x < self.spots[0].len() - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
        }
    }

    fn neighbor_pipe(&self, x: usize, y: usize, dir: Dir) -> Option<Pipe> {
        self.neighbor_coords(x, y, dir)
            .and_then(|(nx, ny)| self.spots[ny][nx].pipe)
    }

    fn coloring_mut(&mut self, x: usize, y: usize) -> &mut Option<Coloring> {
        &mut self.spots[y][x].coloring
    }

    fn infer_pipe(&mut self, sx: usize, sy: usize) -> Result<(), String> {
        let mut dirs = Vec::with_capacity(4);

        if let Some(pipe) = self.neighbor_pipe(sx, sy, Dir::Left) {
            if pipe.connects_to(Dir::Right) {
                dirs.push(Dir::Left);
            }
        }
        if let Some(pipe) = self.neighbor_pipe(sx, sy, Dir::Right) {
            if pipe.connects_to(Dir::Left) {
                dirs.push(Dir::Right);
            }
        }
        if let Some(pipe) = self.neighbor_pipe(sx, sy, Dir::Up) {
            if pipe.connects_to(Dir::Down) {
                dirs.push(Dir::Up);
            }
        }
        if let Some(pipe) = self.neighbor_pipe(sx, sy, Dir::Down) {
            if pipe.connects_to(Dir::Up) {
                dirs.push(Dir::Down);
            }
        }
        if dirs.len() != 2 {
            return Err(format!(
                "Wrong number of pipes connecting. Got these: {:?}",
                dirs
            ));
        }
        self.spots[sy][sx].pipe = Some(Pipe::try_from((dirs[0], dirs[1])).unwrap());

        Ok(())
    }

    fn mark_path_spot(&mut self, x: usize, y: usize, dir: Dir) {
        *self.coloring_mut(x, y) = Some(Coloring::Path);

        if let Some((nx, ny)) = self.neighbor_coords(x, y, dir.rotated_cw()) {
            let coloring = self.coloring_mut(nx, ny);

            if coloring.is_none() {
                *coloring = Some(Coloring::Outside);
            }
        }

        if let Some((nx, ny)) = self.neighbor_coords(x, y, dir.rotated_ccw()) {
            let coloring = self.coloring_mut(nx, ny);

            if coloring.is_none() {
                *coloring = Some(Coloring::Inside);
            }
        }
    }

    fn calc_cycle(&mut self, sx: usize, sy: usize) -> u32 {
        let start = (sx, sy);

        let mut length = 0;
        let mut current = start;
        let mut back = self.spots[sy][sx].pipe.unwrap().dirs().0;
        let mut cw_turns = 0;

        loop {
            let (x, y) = current;
            let here = self.spots[y][x].pipe.unwrap();
            let here_dirs = here.dirs();
            let dir = if here_dirs.0 == back {
                here_dirs.1
            } else {
                here_dirs.0
            };

            let forward = back.opposite();
            if forward.rotated_cw() == dir {
                cw_turns += 1;
            } else if forward.rotated_ccw() == dir {
                cw_turns -= 1;
            }

            self.mark_path_spot(x, y, forward);
            self.mark_path_spot(x, y, dir);

            current = match dir {
                Dir::Up => (x, y - 1),
                Dir::Down => (x, y + 1),
                Dir::Left => (x - 1, y),
                Dir::Right => (x + 1, y),
            };
            back = dir.opposite();
            length += 1;

            if current == start {
                break;
            }
        }

        if cw_turns == 4 {
            self.swap_coloring();
        } else if cw_turns != -4 {
            panic!("unexpected cw turns: {}", cw_turns);
        }

        self.fill_coloring();

        length
    }

    fn swap_coloring(&mut self) {
        for y in 0..self.spots.len() {
            for x in 0..self.spots[0].len() {
                if let Some(coloring) = &mut self.spots[y][x].coloring {
                    *coloring = match *coloring {
                        Coloring::Path => Coloring::Path,
                        Coloring::Inside => Coloring::Outside,
                        Coloring::Outside => Coloring::Inside,
                    }
                }
            }
        }
    }

    fn fill_coloring(&mut self) {
        for y in 0..self.spots.len() {
            for x in 0..self.spots[0].len() {
                if let Some(color) = self.spots[y][x].coloring {
                    if color != Coloring::Path {
                        let mut q: Queue<(usize, usize)> = Queue::new();
                        q.add((x, y)).unwrap();

                        while let Ok((x, y)) = q.remove() {
                            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                                if let Some((nx, ny)) = self.neighbor_coords(x, y, dir) {
                                    let col = self.coloring_mut(nx, ny);
                                    if col.is_none() {
                                        *col = Some(color);
                                        q.add((nx, ny)).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn enclosed_area(&mut self, sx: usize, sy: usize) -> u32 {
        self.calc_cycle(sx, sy);

        self.spots
            .iter()
            .flat_map(|row| row.iter())
            .filter(|spot| spot.coloring == Some(Coloring::Inside))
            .count() as u32
    }
}

fn parse_grid(lines: Vec<&str>) -> (Grid, (usize, usize)) {
    let mut sx = 0;
    let mut sy = 0;
    let mut rows = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for ch in line.chars() {
            if ch == 'S' {
                sx = row.len();
                sy = rows.len();
                row.push(Spot {
                    pipe: None,
                    coloring: None,
                });
            } else if ch == '.' {
                row.push(Spot {
                    pipe: None,
                    coloring: None,
                });
            } else {
                let pipe = Pipe::try_from(ch).unwrap();
                row.push(Spot {
                    pipe: Some(pipe),
                    coloring: None,
                });
            }
        }
        rows.push(row);
    }
    (Grid { spots: rows }, (sx, sy))
}

pub fn run(lines: Vec<&str>, part1: bool) -> u32 {
    let (mut grid, (sx, sy)) = parse_grid(lines);
    grid.infer_pipe(sx, sy).unwrap();
    if part1 {
        grid.calc_cycle(sx, sy) / 2
    } else {
        grid.enclosed_area(sx, sy)
    }
}

use std::collections::HashSet;

struct Board {
    galaxies: Vec<(u32, u32)>,
    rows_not_expanding: HashSet<u32>,
    cols_not_expanding: HashSet<u32>,
}

impl Board {
    fn new(galaxies: Vec<(u32, u32)>) -> Board {
        let rows_not_expanding = galaxies.iter().map(|&(x, _)| x).collect();
        let cols_not_expanding = galaxies.iter().map(|&(_, y)| y).collect();

        Board {
            galaxies,
            rows_not_expanding,
            cols_not_expanding,
        }
    }

    fn adjusted_distance(&self, i1: usize, i2: usize, expansion: u64) -> u64 {
        let (x1, y1) = self.galaxies[i1];
        let (x2, y2) = self.galaxies[i2];

        let mut dist = 0;

        if x1 != x2 {
            dist += 1;

            let rows = if x1 < x2 { (x1 + 1)..x2 } else { (x2 + 1)..x1 };
            for x in rows {
                let expanding = !self.rows_not_expanding.contains(&x);
                dist += if expanding { expansion } else { 1 };
            }
        }

        if y1 != y2 {
            dist += 1;

            let cols = if y1 < y2 { (y1 + 1)..y2 } else { (y2 + 1)..y1 };
            for y in cols {
                let expanding = !self.cols_not_expanding.contains(&y);
                dist += if expanding { expansion } else { 1 };
            }
        }

        dist
    }
}

fn parse_board(lines: Vec<&str>) -> Board {
    let mut galaxies = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((x as u32, y as u32));
            }
        }
    }

    Board::new(galaxies)
}

pub fn run(lines: Vec<&str>, expansion: u64) -> u64 {
    let board = parse_board(lines);

    let mut total = 0;
    for i1 in 0..board.galaxies.len() {
        for i2 in (i1 + 1)..board.galaxies.len() {
            total += board.adjusted_distance(i1, i2, expansion);
        }
    }
    total
}

use itertools::Itertools;

struct Board {
    rows: Vec<Vec<bool>>,
}

impl Board {
    fn horizontal_reflections(&self) -> Vec<usize> {
        let mut refl = Vec::new();

        for y in 0..(self.rows.len() - 1) {
            for d in 0.. {
                let lo = y - d;
                let hi = y + 1 + d;

                if self.rows[lo] != self.rows[hi] {
                    break;
                }
                if lo == 0 || hi == self.rows.len() - 1 {
                    refl.push(y + 1);
                    break;
                }
            }
        }

        refl
    }

    fn fuzzy_horizontal_reflection(&mut self) -> Option<usize> {
        let originals = self.horizontal_reflections();

        for y in 0..self.rows.len() {
            for x in 0..self.rows[0].len() {
                let value = self.rows[y][x];
                self.rows[y][x] = !value;
                let fixed = self.horizontal_reflections();
                self.rows[y][x] = value;

                for idx in fixed {
                    if !originals.contains(&idx) {
                        return Some(idx);
                    }
                }
            }
        }

        None
    }

    fn transposed(&self) -> Board {
        let mut cols = Vec::with_capacity(self.rows[0].len());

        for x in 0..self.rows[0].len() {
            let mut col = Vec::with_capacity(self.rows.len());

            for y in 0..self.rows.len() {
                col.push(self.rows[y][x]);
            }

            cols.push(col);
        }

        Board { rows: cols }
    }
}

fn parse_board(lines: &[&str]) -> Board {
    let mut rows = Vec::new();
    for &line in lines.iter() {
        rows.push(line.chars().map(|ch| ch == '#').collect());
    }
    Board { rows }
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let mut result = 0;

    let mut lines_iter = lines.into_iter();
    loop {
        let board_rows = lines_iter
            .take_while_ref(|s| !s.is_empty())
            .collect::<Vec<_>>();

        lines_iter.next(); // drop the empty line

        if board_rows.is_empty() {
            break;
        }

        let mut board = parse_board(&board_rows);

        if part1 {
            let h: usize = board.horizontal_reflections().iter().sum();
            let v: usize = board.transposed().horizontal_reflections().iter().sum();

            result += h * 100 + v;
        } else {
            let h = board.fuzzy_horizontal_reflection();
            let v = board.transposed().fuzzy_horizontal_reflection();

            if let Some(idx) = h {
                result += idx * 100;
            }
            if let Some(idx) = v {
                result += idx;
            }
        }
    }

    result as u64
}

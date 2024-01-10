use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Empty,
    Cube,  // stone fixed in place
    Round, // rolling stone
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Board {
    rows: Vec<Vec<Tile>>,
}

impl Board {
    fn new(rows: Vec<Vec<Tile>>) -> Board {
        Board { rows }
    }

    fn tilt_north(&mut self) {
        while self.shift_north() {}
    }

    fn shift_north(&mut self) -> bool {
        let mut changed = false;
        for y in 0..(self.rows.len() - 1) {
            for x in 0..self.rows[0].len() {
                if self.rows[y][x] == Tile::Empty && self.rows[y + 1][x] == Tile::Round {
                    self.rows[y][x] = Tile::Round;
                    self.rows[y + 1][x] = Tile::Empty;
                    changed = true;
                }
            }
        }
        changed
    }

    fn rotate_cw(&mut self) {
        let mut cols = Vec::with_capacity(self.rows[0].len());

        for x in 0..self.rows[0].len() {
            let mut col = Vec::with_capacity(self.rows.len());

            for y in 0..self.rows.len() {
                col.push(self.rows[y][x]);
            }

            cols.push(col);
        }

        for col in cols.iter_mut() {
            col.reverse();
        }

        self.rows = cols;
    }

    fn perform_tilt_cycle(&mut self) {
        for _ in 0..4 {
            self.tilt_north();
            self.rotate_cw();
        }
    }

    fn total_load(&self) -> u64 {
        let mut total = 0;

        let height = self.rows.len();
        for y in 0..height {
            for tile in self.rows[y].iter() {
                if *tile == Tile::Round {
                    total += height - y;
                }
            }
        }

        total as u64
    }
}

fn parse_board(lines: &[&str]) -> Board {
    let mut rows = Vec::new();

    for line in lines.iter() {
        rows.push(
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::Empty,
                    'O' => Tile::Round,
                    '#' => Tile::Cube,
                    _ => panic!("invalid character: {}", ch),
                })
                .collect(),
        );
    }

    Board::new(rows)
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    if part1 {
        let mut board = parse_board(&lines);
        board.tilt_north();
        board.total_load()
    } else {
        let mut board = parse_board(&lines);

        let mut cache: HashMap<Board, u64> = HashMap::new();

        let steps = 1000000000;
        for i in 0..steps {
            if let Some(&j) = cache.get(&board) {
                let cycle_len = i - j;
                let steps_left = (steps - i) % cycle_len;
                for _ in 0..steps_left {
                    board.perform_tilt_cycle();
                }
                break;
            }

            cache.insert(board.clone(), i);
            board.perform_tilt_cycle();
        }

        board.total_load()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_board_works() {
        use Tile::*;

        assert_eq!(
            parse_board(&[".O", "#."]),
            Board::new(vec![vec![Empty, Round], vec![Cube, Empty]])
        );
    }

    #[test]
    fn tilted_north_works() {
        use Tile::*;

        let mut board = Board::new(vec![
            vec![Empty, Empty, Empty],
            vec![Cube, Empty, Round],
            vec![Round, Round, Round],
        ]);
        board.tilt_north();

        let expected = Board::new(vec![
            vec![Empty, Round, Round],
            vec![Cube, Empty, Round],
            vec![Round, Empty, Empty],
        ]);

        assert_eq!(board, expected);
    }

    #[test]
    fn rotate_cw_works() {
        use Tile::*;

        let mut board = Board::new(vec![
            vec![Round, Empty], //
            vec![Cube, Round],
        ]);
        board.rotate_cw();

        let expected = Board::new(vec![
            vec![Cube, Round], //
            vec![Round, Empty],
        ]);

        assert_eq!(board, expected);
    }

    #[test]
    fn total_load_works() {
        use Tile::*;

        let board = Board::new(vec![
            vec![Round, Empty], // 2
            vec![Cube, Round],  // 1
        ]);

        assert_eq!(board.total_load(), 2 + 1);
    }
}

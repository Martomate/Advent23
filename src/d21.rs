use std::collections::{HashSet, VecDeque};

use num::Integer;

struct Map {
    width: usize,
    height: usize,
    rows: Vec<Vec<char>>,
}

impl Map {
    fn from_lines(lines: Vec<&str>) -> (Self, (usize, usize)) {
        let width = lines[0].len();
        let height = lines.len();

        let mut rows = Vec::new();
        let mut sx = 0;
        let mut sy = 0;

        for (y, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    sx = x;
                    sy = y;
                    row.push('.');
                } else {
                    row.push(c);
                }
            }
            rows.push(row);
        }

        (
            Self {
                width,
                height,
                rows,
            },
            (sx, sy),
        )
    }
}

pub fn run(lines: Vec<&str>, steps: u32, repeat: bool) -> u64 {
    let (map, (sx, sy)) = Map::from_lines(lines);

    let shell_size = (map.width.lcm(&map.height) * 2) as u32;
    let mut shells = num::integer::div_floor(steps, shell_size);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut q: VecDeque<(i32, i32, u32)> = VecDeque::new();
    q.push_back((sx as i32, sy as i32, 0));

    let mut count: u64 = 0;

    let mut count_per_shell: Vec<u64> = Vec::new();
    let mut next_shell_dist = steps - shells * shell_size;

    while let Some((x, y, dist)) = q.pop_front() {
        if visited.contains(&(x, y)) || dist > steps {
            continue;
        }
        visited.insert((x, y));

        if dist > next_shell_dist {
            count_per_shell.push(count);

            if count_per_shell.len() >= 4 {
                let last3 = &count_per_shell[(count_per_shell.len() - 4)..];

                let diff0 = last3[1] - last3[0];
                let diff1 = last3[2] - last3[1];
                let diff2 = last3[3] - last3[2];

                let acc0 = diff1 - diff0;
                let acc1 = diff2 - diff1;

                if acc0 == acc1 {
                    break;
                }
            }

            if count_per_shell.len() > 100 {
                panic!("could not find stable pattern");
            }

            next_shell_dist += shell_size;
            shells -= 1;
        }

        if (steps - dist) % 2 == 0 {
            count += 1;
        }

        for i in 0..4 {
            let i = 2 * i + 1;
            let dx: i32 = (i % 3) - 1;
            let dy: i32 = (i / 3) - 1;

            let nx = x + dx;
            let ny = y + dy;

            if repeat {
                let h = map.height as i32;
                let w = map.width as i32;
                if map.rows[((ny % h + h) % h) as usize][((nx % w + w) % w) as usize] == '.' {
                    q.push_back((nx, ny, dist + 1));
                }
            } else {
                let in_bounds =
                    ny >= 0 && nx >= 0 && ny < map.height as i32 && nx < map.width as i32;
                if in_bounds && map.rows[ny as usize][nx as usize] == '.' {
                    q.push_back((nx, ny, dist + 1));
                }
            }
        }
    }

    if repeat && shells > 0 {
        let last3 = &count_per_shell[(count_per_shell.len() - 3)..];

        let diff0 = last3[1] - last3[0];
        let diff1 = last3[2] - last3[1];

        let acc = diff1 - diff0;

        let start = diff1 + acc;

        let extra = (start + (start + acc * (shells - 1) as u64)) / 2;

        count += extra * shells as u64;
    }

    count
}

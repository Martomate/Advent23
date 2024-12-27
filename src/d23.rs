mod dir {
    use std::ops::{Index, IndexMut};

    pub const UP: usize = 0;
    pub const RIGHT: usize = 1;
    pub const DOWN: usize = 2;
    pub const LEFT: usize = 3;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Dirs([bool; 4]);

    impl Dirs {
        pub fn all() -> Self {
            Self([true; 4])
        }

        pub fn none() -> Self {
            Self([false; 4])
        }

        pub fn with(mut self, dir: usize) -> Self {
            self.0[dir] = true;
            self
        }
    }

    impl Index<usize> for Dirs {
        type Output = bool;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl IndexMut<usize> for Dirs {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.0[index]
        }
    }
}

use std::iter;

use dir::*;

struct Graph {
    width: usize,
    height: usize,
    edges: Vec<Dirs>,
}

impl Graph {
    fn from_lines(lines: &[&str], ignore_slopes: bool) -> Result<Graph, &'static str> {
        let width = lines[0].len();
        let height = lines.len();

        let cells: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

        let mut edges = Vec::with_capacity(width * height);

        for y in 0..cells.len() {
            for x in 0..width {
                let here = cells[y][x];

                // Step 1: where can we go based on this cell
                let mut e = if !ignore_slopes {
                    match here {
                        '#' => Dirs::none(),
                        '.' => Dirs::all(),
                        '^' => Dirs::none().with(UP),
                        '>' => Dirs::none().with(RIGHT),
                        'v' => Dirs::none().with(DOWN),
                        '<' => Dirs::none().with(LEFT),
                        _ => return Err("unknown character"),
                    }
                } else {
                    match here {
                        '#' => Dirs::none(),
                        '.' => Dirs::all(),
                        '^' => Dirs::all(),
                        '>' => Dirs::all(),
                        'v' => Dirs::all(),
                        '<' => Dirs::all(),
                        _ => return Err("unknown character"),
                    }
                };

                // Step 2: where can we not actually go based on the neighbors
                if y > 0 && e[UP] {
                    let c = cells[y - 1][x];
                    if c == '#' {
                        e[UP] = false;
                    }
                }
                if x > 0 && e[LEFT] {
                    let c = cells[y][x - 1];
                    if c == '#' {
                        e[LEFT] = false;
                    }
                }
                if y < height - 1 && e[DOWN] {
                    let c = cells[y + 1][x];
                    if c == '#' {
                        e[DOWN] = false;
                    }
                }
                if x < width - 1 && e[RIGHT] {
                    let c = cells[y][x + 1];
                    if c == '#' {
                        e[RIGHT] = false;
                    }
                }

                edges.push(e);
            }
        }

        Ok(Graph {
            width,
            height,
            edges,
        })
    }

    fn find_src(&self) -> Option<usize> {
        self.edges
            .iter()
            .take(self.width)
            .position(|e| *e != Dirs::none())
    }

    fn find_dst(&self) -> Option<usize> {
        self.edges[(self.width * (self.height - 1))..]
            .iter()
            .take(self.width)
            .position(|e| *e != Dirs::none())
    }
}

struct Search {
    graph: Graph,
    visited: Vec<bool>,
}

impl Search {
    fn new(graph: Graph) -> Self {
        let visited = iter::repeat(false)
            .take(graph.width * graph.height)
            .collect();
        Self { graph, visited }
    }

    fn find_longest_path(
        &mut self,
        (src_x, src_y): (usize, usize),
        (dst_x, dst_y): (usize, usize),
    ) -> Option<u64> {
        if src_x == dst_x && src_y == dst_y {
            return Some(0);
        }

        let e = self.graph.edges[src_y * self.graph.width + src_x];

        let mut best: Option<u64> = None;

        for d in [UP, RIGHT, DOWN, LEFT] {
            if e[d] {
                let (dx, dy) = match d {
                    UP => (0, -1),
                    RIGHT => (1, 0),
                    DOWN => (0, 1),
                    LEFT => (-1, 0),
                    _ => unreachable!(),
                };

                let nx = src_x as isize + dx;
                let ny = src_y as isize + dy;

                if nx < 0
                    || ny < 0
                    || nx >= self.graph.width as isize
                    || ny >= self.graph.height as isize
                {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;
                let idx = ny * self.graph.width + nx;

                if !self.visited[idx] {
                    self.visited[idx] = true;
                    if let Some(res) = self.find_longest_path((nx, ny), (dst_x, dst_y)) {
                        if best.is_none() || best.unwrap() < res {
                            best = Some(res);
                        }
                    }
                    self.visited[idx] = false;
                }
            }
        }

        best.map(|d| d + 1)
    }
}

pub fn run(lines: Vec<&str>, ignore_slopes: bool) -> u64 {
    let graph = Graph::from_lines(&lines, ignore_slopes).unwrap();
    let src_x = graph.find_src().unwrap();
    let dst_x = graph.find_dst().unwrap();

    let src = (src_x, 0);
    let dst = (dst_x, graph.height - 1);

    let mut search = Search::new(graph);
    search.find_longest_path(src, dst).unwrap()
}

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

use dir::*;

struct BasicGraph {
    width: usize,
    height: usize,
    edges: Vec<Dirs>,
}

impl BasicGraph {
    fn from_lines(lines: &[&str], ignore_slopes: bool) -> Result<BasicGraph, &'static str> {
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

        Ok(BasicGraph {
            width,
            height,
            edges,
        })
    }

    fn as_compact_graph(&self) -> Graph {
        let mut nodes = Vec::new();
        let mut node_indices = Vec::with_capacity(self.edges.len());

        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                let c = self.edges[i];
                if c != Dirs::none() {
                    let idx = nodes.len();
                    nodes.push(Node::new());
                    node_indices.push(idx);
                } else {
                    node_indices.push(0);
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                let c = self.edges[i];
                if c != Dirs::none() {
                    let node = &mut nodes[node_indices[i]];

                    for d in [UP, RIGHT, DOWN, LEFT] {
                        if c[d] {
                            let (dx, dy) = match d {
                                UP => (0, -1),
                                RIGHT => (1, 0),
                                DOWN => (0, 1),
                                LEFT => (-1, 0),
                                _ => unreachable!(),
                            };

                            let nx = x as isize + dx;
                            let ny = y as isize + dy;

                            if nx < 0
                                || ny < 0
                                || nx >= self.width as isize
                                || ny >= self.height as isize
                            {
                                continue;
                            }

                            let nx = nx as usize;
                            let ny = ny as usize;
                            let idx = ny * self.width + nx;

                            node.edges[node.num_edges as usize] = (node_indices[idx] as u16, 1);
                            node.num_edges += 1;
                        }
                    }
                }
            }
        }

        Graph { nodes }.simplified()
    }
}

struct Node {
    edges: [(u16, u16); 4], // indices into the `nodes` vector in `Graph`, and the distance
    num_edges: u8,
}

impl Node {
    fn new() -> Self {
        Node {
            edges: [(0, 0); 4],
            num_edges: 0,
        }
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn simplified(mut self) -> Self {
        for j in 0..self.nodes.len() {
            let node = &mut self.nodes[j];
            if node.num_edges == 2 {                
                let (n1, d1) = node.edges[0];
                let (n2, d2) = node.edges[1];
                let dist = d1 + d2;

                node.num_edges = 0;

                let n = &mut self.nodes[n1 as usize];
                for i in 0..n.num_edges {
                    if n.edges[i as usize].0 as usize == j {
                        n.edges[i as usize] = (n2, dist);
                    }
                }
                
                let n = &mut self.nodes[n2 as usize];
                for i in 0..n.num_edges {
                    if n.edges[i as usize].0 as usize == j {
                        n.edges[i as usize] = (n1, dist);
                    }
                }
            }
        }
        self
    }
}

struct Search {
    graph: Graph,
    visited: Vec<bool>,
}

impl Search {
    fn new(graph: Graph) -> Self {
        let visited = std::iter::repeat_n(false, graph.nodes.len()).collect();
        Self { graph, visited }
    }

    fn find_longest_path(&mut self, src: u16, dst: u16) -> Option<u16> {
        if src == dst {
            return Some(0);
        }

        let mut best: Option<u16> = None;

        let Node { edges, num_edges } = self.graph.nodes[src as usize];

        for n_idx in 0..num_edges {
            let (neighbor, dist) = edges[n_idx as usize];
            if !self.visited[neighbor as usize] {
                self.visited[neighbor as usize] = true;
                if let Some(res) = self.find_longest_path(neighbor, dst) {
                    let res = res + dist;
                    if best.is_none() || best.unwrap() < res {
                        best = Some(res);
                    }
                }
                self.visited[neighbor as usize] = false;
            }
        }

        best
    }
}

pub fn run(lines: Vec<&str>, ignore_slopes: bool) -> u64 {
    let graph = BasicGraph::from_lines(&lines, ignore_slopes).unwrap();

    let graph = graph.as_compact_graph();

    let src = 0;
    let dst = (graph.nodes.len() - 1) as u16;

    let mut search = Search::new(graph);
    search.find_longest_path(src, dst).unwrap() as u64
}

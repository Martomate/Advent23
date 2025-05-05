use std::collections::HashMap;

use priority_queue::PriorityQueue;

struct NameCache<'a> {
    indices: HashMap<&'a str, usize>,
    names: Vec<&'a str>,
}

impl<'a> NameCache<'a> {
    fn new() -> Self {
        Self {
            indices: HashMap::new(),
            names: Vec::new(),
        }
    }

    fn insert(&mut self, name: &'a str) -> usize {
        match self.indices.get(name) {
            Some(idx) => *idx,
            None => {
                let idx = self.names.len();
                self.names.push(name);
                self.indices.insert(name, idx);
                idx
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Vec<usize>>,
    active_nodes: Vec<usize>,
    node_cardinalities: Vec<usize>,
}

impl Graph {
    fn new(nodes: Vec<Vec<usize>>) -> Self {
        let active_nodes: Vec<usize> = (0..nodes.len()).filter(|&i| !nodes[i].is_empty()).collect();
        let node_cardinalities: Vec<usize> = std::iter::repeat(1).take(nodes.len()).collect();
        Self {
            nodes,
            active_nodes,
            node_cardinalities,
        }
    }

    fn min_cut(&self) -> MinCut {
        StoerWagner::init(self.clone()).calculate()
    }

    fn merge_nodes(&mut self, s: usize, t: usize) {
        assert_ne!(s, t);

        while let Some(idx) = self.nodes[s].iter().position(|n| *n == t) {
            self.nodes[s].swap_remove(idx);
        }
        while let Some(idx) = self.nodes[t].iter().position(|n| *n == s) {
            self.nodes[t].swap_remove(idx);
        }

        while let Some(n) = self.nodes[t].pop() {
            self.nodes[s].push(n);
            for m in self.nodes[n].iter_mut() {
                if *m == t {
                    *m = s;
                }
            }
        }
        self.active_nodes
            .swap_remove(self.active_nodes.iter().position(|n| *n == t).unwrap());

        self.node_cardinalities[s] += self.node_cardinalities[t];
        self.node_cardinalities[t] = 0;
    }
}

struct MinCut {
    cut_weight: u64,
    split_size: usize,
}

// Stoer–Wagner minimum cut algorithm
struct StoerWagner {
    graph: Graph,
}

impl StoerWagner {
    fn init(graph: Graph) -> Self {
        Self { graph }
    }

    fn calculate(mut self) -> MinCut {
        let mut best_cut = usize::MAX;
        let mut best_cut_graph_size: Option<usize> = None;

        let mut num_nodes_left = self.graph.nodes.len();
        while num_nodes_left > 1 {
            let (s, t) = self.min_cut_phase();
            let cut = self.graph.nodes[t].len();
            let cut_graph_size = self.graph.node_cardinalities[t];

            self.graph.merge_nodes(s, t);

            if cut < best_cut {
                best_cut = cut;
                best_cut_graph_size = Some(cut_graph_size);
            }
            num_nodes_left -= 1;
        }

        MinCut {
            cut_weight: best_cut as u64,
            split_size: best_cut_graph_size.unwrap(),
        }
    }

    fn min_cut_phase(&self) -> (usize, usize) {
        let mut most_connected: PriorityQueue<u16, u16> = PriorityQueue::new();
        most_connected.push(0, 0);

        let mut added = Vec::new();

        while let Some((a, _)) = most_connected.pop() {
            added.push(a);
            for n in self.graph.nodes[a as usize].iter() {
                let n = *n as u16;
                if added.contains(&n) {
                    continue;
                }
                if !most_connected.change_priority_by(&n, |c| *c += 1) {
                    most_connected.push(n, 1);
                }
            }
        }

        (
            added[added.len() - 2] as usize,
            added[added.len() - 1] as usize,
        )
    }
}

pub fn run(lines: Vec<&str>) -> u64 {
    let mut names = NameCache::new();

    let mut nodes: Vec<Vec<usize>> = Vec::new();

    for line in lines {
        let (l_str, rs) = line.split_once(':').unwrap();
        let l = names.insert(l_str);

        if l == nodes.len() {
            nodes.push(Vec::new());
        }

        for r_str in rs.trim().split(' ') {
            let r = names.insert(r_str);

            if r == nodes.len() {
                nodes.push(Vec::new());
            }

            nodes[l].push(r);
            nodes[r].push(l);
        }
    }

    let num_nodes = nodes.len();

    let graph = Graph::new(nodes);

    let MinCut {
        cut_weight,
        split_size,
    } = graph.min_cut();

    assert_eq!(cut_weight, 3);

    split_size as u64 * (num_nodes - split_size) as u64
}

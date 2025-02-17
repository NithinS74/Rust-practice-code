use std::cmp::Ordering;

#[derive(Clone)]
struct Edge {
    src: usize,
    dest: usize,
    weight: f32,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] != node {
            self.parent[node] = self.find(self.parent[node]);
        }
        self.parent[node]
    }

    fn union(&mut self, u: usize, v: usize) {
        let root_u = self.find(u);
        let root_v = self.find(v);

        if root_u != root_v {
            if self.rank[root_u] > self.rank[root_v] {
                self.parent[root_v] = root_u;
            } else if self.rank[root_u] < self.rank[root_v] {
                self.parent[root_u] = root_v;
            } else {
                self.parent[root_v] = root_u;
                self.rank[root_u] += 1;
            }
        }
    }
}

fn kruskal(num_nodes: usize, mut edges: Vec<Edge>) -> Vec<Edge> {
    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap_or(Ordering::Equal));
    let mut uf = UnionFind::new(num_nodes);
    let mut mst = Vec::new();

    for edge in edges {
        if uf.find(edge.src) != uf.find(edge.dest) {
            uf.union(edge.src, edge.dest);
            mst.push(edge);
        }
    }

    mst
}

pub fn kruskal_algorithm() {
    let edges = vec![
        Edge {
            src: 0,
            dest: 1,
            weight: 4.0,
        },
        Edge {
            src: 0,
            dest: 2,
            weight: 4.5,
        },
        Edge {
            src: 1,
            dest: 2,
            weight: 2.2,
        },
        Edge {
            src: 1,
            dest: 3,
            weight: 5.1,
        },
        Edge {
            src: 2,
            dest: 3,
            weight: 5.0,
        },
        Edge {
            src: 3,
            dest: 4,
            weight: 3.3,
        },
    ];

    let mst = kruskal(5, edges);

    for edge in mst {
        println!(
            "Edge: {} - {} with weight {}",
            edge.src, edge.dest, edge.weight
        );
    }
}

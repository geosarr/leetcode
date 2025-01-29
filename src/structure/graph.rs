use crate::Number;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

use super::Queue;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WeightedEdge<T> {
    pub src: usize,
    pub dest: usize,
    pub weight: T,
    pub num: usize,
}

impl<T> WeightedEdge<T> {
    pub fn new(src: usize, dest: usize, weight: T, num: usize) -> Self {
        Self {
            src,
            dest,
            weight,
            num,
        }
    }
    pub fn other_endpoint(&self, vertex: usize) -> usize {
        if self.src == vertex {
            return self.dest;
        } else if self.dest == vertex {
            return self.src;
        } else {
            panic!("Illegal endpoint {vertex}");
        }
    }
    pub fn residual_capacity(&self, flows: &[T], vertex: usize) -> T
    where
        T: Number,
    {
        if self.src == vertex {
            // backward edge
            flows[self.num]
        } else if self.dest == vertex {
            // forward edge
            self.weight - flows[self.num]
        } else {
            panic!("Illegal endpoint {vertex}");
        }
    }
    pub fn adjust_path_flow(&self, flows: &mut [T], vertex: usize, delta: T)
    where
        T: Number,
    {
        if self.src == vertex {
            // backward edge
            flows[self.num] = flows[self.num] - delta
        } else if self.dest == vertex {
            // forward edge
            flows[self.num] = flows[self.num] + delta
        } else {
            panic!("Illegal endpoint {vertex}");
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeWeightedDiGraph<T: Hash + Eq> {
    pub out_edges: Vec<std::collections::HashSet<WeightedEdge<T>>>,
    pub nb_edges: usize,
    pub nb_vertices: usize,
}
impl<T: Display> Display for WeightedEdge<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ---> {}; {})", self.src, self.dest, self.weight)
    }
}
impl<T: Display + Hash + Eq> Display for EdgeWeightedDiGraph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for edges in &self.out_edges {
            for edge in edges {
                write!(f, "{edge}, ")?;
            }
            if !edges.is_empty() {
                writeln!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl<T: Hash + Eq> EdgeWeightedDiGraph<T> {
    pub fn new() -> Self {
        Self {
            out_edges: vec![],
            nb_edges: 0,
            nb_vertices: 0,
        }
    }
    pub fn add_edge(&mut self, src: usize, dest: usize, weight: T) {
        let max = std::cmp::max(src, dest);
        if self.nb_vertices > max {
            self.out_edges[src].insert(WeightedEdge::new(src, dest, weight, self.nb_edges));
            self.nb_edges += 1;
        } else {
            let nb = max - self.nb_vertices + 1;
            for _ in 0..nb {
                self.out_edges.push(HashSet::new());
            }
            self.nb_vertices += nb;
            self.add_edge(src, dest, weight);
        }
    }
    // pub fn in_edges(&self, dest: usize) -> impl Iterator<Item = &WeightedEdge<T>> {
    //     self.out_edges
    //         .iter()
    //         .flat_map(|out| out.iter().filter(|edge| edge.dest == dest))
    // }
    pub fn from_iter<E>(edges: E) -> Self
    where
        E: IntoIterator<Item = (usize, usize, T)>,
    {
        let mut graph = Self::new();
        for (src, dest, weight) in edges {
            graph.add_edge(src, dest, weight);
        }
        graph
    }
    pub fn dfs(
        &self,
        src: usize,
        marked: &mut Vec<bool>,
        edge_to: &mut Vec<usize>,
        mut_edge_to_at_end: bool,
    ) {
        marked[src] = true;
        if !mut_edge_to_at_end {
            for neighbor in self.out_edges[src]
                .iter()
                .map(|edge| edge.other_endpoint(src))
            {
                if !marked[neighbor] {
                    self.dfs(neighbor, marked, edge_to, mut_edge_to_at_end);
                    edge_to[neighbor] = src;
                }
            }
        } else {
            for neighbor in self.out_edges[src]
                .iter()
                .map(|edge| edge.other_endpoint(src))
            {
                if !marked[neighbor] {
                    self.dfs(neighbor, marked, edge_to, mut_edge_to_at_end);
                }
            }
            edge_to.push(src); // For topological sort algorithm.
        }
    }
    pub fn depth_first_order(&self) -> (Vec<bool>, Vec<usize>) {
        let mut marked = vec![false; self.nb_vertices];
        let mut reverse_postorder = vec![];
        for v in 0..self.nb_vertices {
            if !marked[v] {
                self.dfs(v, &mut marked, &mut reverse_postorder, true);
            }
        }
        (marked, reverse_postorder)
    }
    pub fn shorted_path_ewdag(&self, source: usize) -> (Vec<T>, Vec<usize>)
    where
        T: Number + Clone,
    {
        let mut dist_to = vec![T::max(); self.nb_vertices];
        let mut edge_to = vec![usize::MAX; self.nb_vertices];
        let mut flag_source = false;
        dist_to[source] = T::zero();
        for vertex in self.depth_first_order().1.iter().rev().copied() {
            if vertex == source {
                flag_source = true;
            }
            if flag_source {
                for (neighbor, dist_vertex_neighbor) in self.out_edges[vertex]
                    .iter()
                    .map(|edge| (edge.other_endpoint(vertex), edge.weight))
                {
                    let new_dist = dist_to[vertex] + dist_vertex_neighbor;
                    if dist_to[neighbor] > new_dist {
                        dist_to[neighbor] = new_dist;
                        edge_to[neighbor] = vertex;
                    }
                }
            }
        }
        (dist_to, edge_to)
    }
    pub fn has_augmented_path(
        &self,
        src: usize,
        dest: usize,
        flows: &mut [T],
    ) -> (Vec<Option<WeightedEdge<T>>>, bool)
    where
        T: Number + std::fmt::Debug,
    {
        let mut marked = vec![false; self.nb_vertices];
        let mut edge_to = vec![None; self.nb_vertices];
        marked[src] = true;
        let mut queue = Queue::new();
        queue.push(src);
        let mut vertex = src;
        while let (q, vrtx) = queue.pop() {
            queue = q;
            vertex = if let Some(v) = vrtx {
                v
            } else {
                break;
            };
            // Union of incoming edges to vertex and outgoing edges from vertex.
            let edges = self.out_edges[vertex].iter().chain(
                self.out_edges
                    .iter()
                    .flat_map(|out| out.iter().filter(|edge| edge.dest == vertex)),
            );
            for edge in edges {
                let next = edge.other_endpoint(vertex);
                if !marked[next] && edge.residual_capacity(&*flows, next) > T::zero() {
                    marked[next] = true;
                    edge_to[next] = Some(*edge);
                    if next == dest {
                        return (edge_to, true);
                    }
                    queue.push(next);
                }
            }
        }
        (edge_to, false)
    }
    pub fn ford_fulkerson(&self, src: usize, dest: usize) -> (Vec<T>, T)
    where
        T: Number + std::fmt::Debug,
    {
        let mut max_flow = T::zero();
        let mut flows = vec![T::zero(); self.nb_edges];
        let (mut edge_to, mut exists_aug_path) = self.has_augmented_path(src, dest, &mut flows);
        println!("{exists_aug_path}");
        while exists_aug_path {
            let mut path_flow = T::max();

            // Find the bottleneck capacity
            let mut vertex = dest;
            while let Some(edge) = &edge_to[vertex] {
                let res_cap = edge.residual_capacity(&flows, vertex);
                path_flow = if res_cap < path_flow {
                    res_cap
                } else {
                    path_flow
                };
                vertex = edge.other_endpoint(vertex);
            }

            // Update the network flows
            let mut vertex = dest;
            while let Some(edge) = &edge_to[vertex] {
                edge.adjust_path_flow(&mut flows, vertex, path_flow);
                vertex = edge.other_endpoint(vertex);
            }

            // Update the maximum flow.
            max_flow = max_flow + path_flow;
            (edge_to, exists_aug_path) = self.has_augmented_path(src, dest, &mut flows);
        }
        return (flows, max_flow);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_shortest_path_toposort() {
        let graph = EdgeWeightedDiGraph::from_iter([
            (0usize, 1, 1),
            (0, 2, 1),
            (1, 2, 100),
            (1, 3, 100),
            (2, 3, 1),
            (2, 4, 1),
            (3, 4, 1),
            (3, 5, 1),
            (4, 5, 1),
            (4, 6, 1),
            (5, 6, 100),
            (5, 7, 100),
            (6, 7, 1),
            (6, 8, 1),
            (7, 8, 1),
            (7, 9, 1),
            (8, 9, 100),
            (8, 10, 100),
            (9, 10, 1),
        ]);
        println!("{:?}", graph);
        let (dist_to, edge_to) = graph.shorted_path_ewdag(1usize);
        println!("{:?}", dist_to);
        println!("{:?}", edge_to);
    }

    #[test]
    fn test_graph_ford_fulkerson() {
        let graph = EdgeWeightedDiGraph::from_iter([
            (0usize, 1, 16),
            (0, 2, 13),
            (1, 3, 12),
            (2, 1, 4),
            (2, 4, 14),
            (3, 2, 9),
            (3, 5, 20),
            (4, 3, 7),
            (4, 5, 4),
        ]);
        println!("{:?}", graph.ford_fulkerson(0, 5))
    }
}

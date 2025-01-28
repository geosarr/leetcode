//! Contains Rust solutions for Leetcode problems.

use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Display, Error},
    hash::Hash,
};

/// Contains all easy problems, based on leetcode difficulty levels nomenclature.
pub mod easy;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WeightedEdge<T> {
    pub src: usize,
    pub dest: usize,
    pub weight: T,
}

impl<T> WeightedEdge<T> {
    pub fn new(src: usize, dest: usize, weight: T) -> Self {
        Self { src, dest, weight }
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
pub trait Number
where
    Self: std::ops::Add<Self, Output = Self> + PartialOrd + Sized + Copy,
{
    fn max() -> Self;
    fn zero() -> Self;
}
macro_rules! impl_number {
    ($typ:ty) => {
        impl Number for $typ {
            fn max() -> Self {
                <$typ>::MAX
            }
            fn zero() -> Self {
                0 as Self
            }
        }
    };
}
impl_number!(usize);
impl_number!(i32);
impl_number!(u32);

#[derive(Debug, Clone)]
pub struct QueueNode {
    value: i32,
    next: Option<Box<QueueNode>>,
}
impl QueueNode {
    pub fn new(value: i32) -> Self {
        Self { value, next: None }
    }
}

#[derive(Debug)]
pub struct Queue {
    first: Option<Box<QueueNode>>, // first to exit the queue
    len: usize,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            first: None,
            len: 0,
        }
    }
    pub fn init(value: i32) -> Self {
        Self {
            first: Some(Box::new(QueueNode::new(value))),
            len: 1,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn push(&mut self, value: i32) {
        if self.is_empty() {
            self.first = Some(Box::new(QueueNode::new(value)));
        } else {
            let mut node = self.first.as_mut().unwrap();
            while let Some(ref mut nod) = node.next {
                node = nod;
            }
            node.next = Some(Box::new(QueueNode::new(value)));
        }
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        let res = self.first.as_ref().unwrap().value;
        self.first = self.first.as_ref().unwrap().next.clone();
        Some(res)
    }
}

impl Display for QueueNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref node) = self.next {
            write!(f, "{} -> {}", self.value, node)
        } else {
            write!(f, "{}", self.value)
        }
    }
}
impl Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref first) = self.first {
            first.fmt(f)
        } else {
            Err(Error)
        }
    }
}
#[cfg(test)]
mod tests_queue {
    use super::*;

    #[test]
    fn test_graph() {
        let mut queue = Queue::init(0);
        queue.push(1);
        queue.push(2);
        println!("{}", queue);
        queue.pop();
        println!("{}", queue);
        queue.pop();
        println!("{}", queue);
        queue.pop();
        println!("{}", queue);
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
            self.out_edges[src].insert(WeightedEdge::new(src, dest, weight));
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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
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
}
#[derive(Clone, Debug, PartialEq)]
struct Node<T, U> {
    key: T,
    value: U,
    left: Option<Box<Node<T, U>>>,
    right: Option<Box<Node<T, U>>>,
}
impl<T, U> Node<T, U> {
    pub fn init(_key: T, _value: U) -> Self {
        Self {
            key: _key,
            value: _value,
            left: None,
            right: None,
        }
    }
}

/// Implementation of a binary search tree
/// # Example
/// ```
/// use leetcode::BSearchTree;
/// let mut bt = BSearchTree::new();
/// bt.insert(0,"1");
/// bt.insert(1,"2");
/// bt.insert(2,"3");
/// assert_eq!(bt.len(), 3);
/// assert!(bt.contains(&0));
/// assert_eq!(bt.get(&2), Some(&"3"));
/// ```
#[derive(Debug, Clone)]
pub struct BSearchTree<T, U> {
    root: Option<Box<Node<T, U>>>,
    len: usize,
}
impl<T, U> Default for BSearchTree<T, U> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T, U> BSearchTree<T, U> {
    /// Creates an empty tree instance.
    /// # Example
    /// ```
    /// use leetcode::BSearchTree;
    /// let bt = BSearchTree::<usize, isize>::new();
    /// assert_eq!(bt.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }
    /// Creates a new tree with an initial (key, value) pair.
    /// # Example
    /// ```
    /// use leetcode::BSearchTree;
    /// let bt = BSearchTree::init("btree", 0);
    /// assert_eq!(bt.len(), 1);
    /// ```
    pub fn init(key: T, value: U) -> Self {
        Self {
            root: Some(Box::new(Node::init(key, value))),
            len: 1,
        }
    }
    /// Gives the number of (key, value) pairs in the tree.
    /// # Example
    /// ```
    /// use leetcode::BSearchTree;
    /// let bt = BSearchTree::<usize, usize>::new();
    /// assert_eq!(bt.len(),0);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }
    /// Tests whether or not the tree is empty.
    /// # Example
    /// ```
    /// use leetcode::BSearchTree;
    /// let mut bt = BSearchTree::new();
    /// bt.insert(1,1);
    /// assert!(!bt.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<T: Eq + Ord, U: Eq> BSearchTree<T, U> {
    /// Tests whether or not the tree contains a given key.
    /// # Example
    /// ```
    /// use leetcode::BSearchTree;
    /// let bt = BSearchTree::init("btree", "one");
    /// assert!(bt.contains(&"btree"));
    /// ```
    pub fn contains(&self, key: &T) -> bool {
        self.get(key).is_some()
    }
    /// Returns a reference of the value associated to a key if any exists in the tree.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use leetcode::BSearchTree;
    /// let bt = BSearchTree::init("btree", "one");
    /// assert_eq!(bt.get(&"no btree"), None);
    /// assert_eq!(bt.get(&"btree"), Some(&"one"));
    /// ```
    pub fn get(&self, key: &T) -> Option<&U> {
        // gets the value associated to key if key is in
        // the tree, otherwise returns None,
        // run time complexity on average O(log(N)), O(N) guaranteed (unbalanced tree)
        let mut node = &self.root;
        while node.is_some() {
            let temp_node = node.as_ref().unwrap();
            match key.cmp(&temp_node.key) {
                Ordering::Less => node = &temp_node.left,
                Ordering::Greater => node = &temp_node.right,
                Ordering::Equal => return Some(&temp_node.value),
            }
        }
        None
    }
}
impl<T: Ord, U> BSearchTree<T, U> {
    fn put(node: &mut Option<Box<Node<T, U>>>, key: T, value: U) -> Option<&U> {
        match node {
            None => *node = Some(Box::new(Node::init(key, value))),
            Some(ref mut nod) => match key.cmp(&nod.key) {
                Ordering::Less => {
                    return Self::put(&mut nod.left, key, value);
                }
                Ordering::Greater => {
                    return Self::put(&mut nod.right, key, value);
                }
                Ordering::Equal => {
                    nod.value = value;
                    return Some(&nod.value);
                }
            },
        }
        None
    }
    /// Inserts a (key, value) pair in the tree. When the input key is
    /// already on the map, then it replaces the old value with the new one specified.   
    /// # Example
    /// ```
    /// use leetcode::BSearchTree;
    /// let mut bt = BSearchTree::<isize, usize>::new();
    /// bt.insert(-1, 2);
    /// bt.insert(-2, 3);
    /// bt.insert(-1, 4);
    /// assert_eq!(bt.len(), 2);
    /// assert_eq!(bt.get(&-2), Some(&3));
    /// ```
    pub fn insert(&mut self, key: T, value: U) {
        if Self::put(&mut self.root, key, value).is_none() {
            self.len += 1;
        }
    }
}

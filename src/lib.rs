//! Contains Rust solutions for Leetcode problems.

use std::cmp::Ordering;

/// Contains all easy problems, based on leetcode difficulty levels nomenclature.
pub mod easy;

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

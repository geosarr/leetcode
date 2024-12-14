use std::{cell::RefCell, rc::Rc};

/// Given the root of a binary tree, return its maximum depth.
///
/// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
///
/// Example 1:
/// - Input: root = [3,9,20,null,null,15,7]
/// - Output: 3
///
/// Example 2:
/// - Input: root = [1,null,2]
/// - Output: 2
///
/// Constraints:
/// - The number of nodes in the tree is in the range [0, 104].
/// - -100 <= Node.val <= 100
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let depth = 1;
    let tree = root.unwrap();
    return std::cmp::max(
        count(tree.borrow().left.clone(), depth),
        count(tree.borrow().right.clone(), depth),
    );
}

pub fn count(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
    if root.is_none() {
        return depth;
    }
    let depth = depth + 1;
    let tree = root.unwrap();
    return std::cmp::max(
        count(tree.borrow().left.clone(), depth),
        count(tree.borrow().right.clone(), depth),
    );
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

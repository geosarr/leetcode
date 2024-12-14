use std::cell::RefCell;
use std::rc::Rc;

/// Given the root of a binary tree, return the inorder traversal of its nodes' values.
///
/// Example 1:
/// - Input: root = [1,null,2,3]
/// - Output: [1,3,2]
///
/// Example 2:
/// - Input: root = [1,2,3,4,5,null,8,null,null,6,7,9]
/// - Output: [4,2,6,5,7,1,3,9,8]
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }
    let mut res = Vec::new();
    let tree = root.unwrap();
    let root_val = tree.borrow().val;
    let left = inorder_traversal(tree.borrow().left.clone());
    let right = inorder_traversal(tree.borrow().right.clone());
    res.extend_from_slice(&left);
    res.push(root_val);
    res.extend_from_slice(&right);
    res
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

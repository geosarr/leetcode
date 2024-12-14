use std::{cell::RefCell, rc::Rc};

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let tree = root.unwrap();
    if tree.borrow().left.is_none() && tree.borrow().right.is_none() {
        return true;
    }
    if tree.borrow().left.is_some() && tree.borrow().right.is_none() {
        return false;
    }
    if tree.borrow().left.is_none() && tree.borrow().right.is_some() {
        return false;
    }
    // AT this left and right have values
    if tree.borrow().left.as_ref().unwrap().borrow().val
        != tree.borrow().right.as_ref().unwrap().borrow().val
    {
        return false;
    }
    let left_traversal = inorder_traversal(tree.borrow().left.clone(), true);
    let right_traversal = inorder_traversal(tree.borrow().right.clone(), false);
    return left_traversal == right_traversal;
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, from_left: bool) -> Vec<Option<i32>> {
    if root.is_none() {
        return vec![];
    }
    let mut res = Vec::new();
    let tree = root.unwrap();
    let root_val = tree.borrow().val;
    let (mut left, mut right) = (
        inorder_traversal(tree.borrow().left.clone(), from_left),
        inorder_traversal(tree.borrow().right.clone(), from_left),
    );
    if right.is_empty() && !left.is_empty() {
        right = vec![None];
    }
    if !right.is_empty() && left.is_empty() {
        left = vec![None];
    }
    if from_left {
        res.extend_from_slice(&left);
        res.push(Some(root_val));
        res.extend_from_slice(&right);
    } else {
        res.extend_from_slice(&right);
        res.push(Some(root_val));
        res.extend_from_slice(&left);
    }
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_symmetric() {
        assert_eq!(is_symmetric(None), true);
        assert_eq!(
            is_symmetric(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            })))),
            true
        );
        assert_eq!(
            is_symmetric(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    })))
                })))
            })))),
            true
        );
        assert_eq!(
            is_symmetric(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            })))),
            false
        );

        assert_eq!(
            is_symmetric(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None
                    }))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            })))),
            false
        );
    }
}

/// You are given the heads of two sorted linked lists list1 and list2.
///
/// Merge the two lists into one sorted list.
///
/// The list should be made by splicing together the nodes of the first two lists.
///
/// Return the head of the merged linked list.
///
/// Example 1:
///
/// - Input: list1 = [1,2,4], list2 = [1,3,4]
/// - Output: [1,1,2,3,4,4]
///
/// Example 2:
///
/// - Input: list1 = [], list2 = []
/// - Output: []
///
/// Example 3:
///
/// - Input: list1 = [], list2 = [0]
/// - Output: [0]
///  
/// Constraints:
///
/// - The number of nodes in both lists is in the range [0, 50].
/// - -100 <= Node.val <= 100
/// - Both list1 and list2 are sorted in non-decreasing order
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    }
    if list2.is_none() {
        return list1;
    }
    let mut l1 = list1;
    let mut l2 = list2;
    let _l1 = l1.as_ref().unwrap();
    let _l2 = l2.as_ref().unwrap();
    let (a, b) = (_l1.val, _l2.val);
    let mut res = if a < b {
        l1 = _l1.next.clone();
        vec![a]
    } else if b < a {
        l2 = _l2.next.clone();
        vec![b]
    } else {
        l1 = _l1.next.clone();
        l2 = _l2.next.clone();
        vec![a, b]
    };
    while l1.is_some() && l2.is_some() {
        // safe to unwrap since `l1` and `l2` are not empty at this stage
        let _l1 = l1.as_ref().unwrap();
        let _l2 = l2.as_ref().unwrap();
        let (a, b) = (_l1.val, _l2.val);
        if a < b {
            l1 = _l1.next.clone();
            res.push(a);
        } else if b < a {
            l2 = _l2.next.clone();
            res.push(b);
        } else {
            l1 = _l1.next.clone();
            l2 = _l2.next.clone();
            res.extend_from_slice(&[a, b]);
        }
    }
    while let Some(node) = l1 {
        res.push(node.val);
        l1 = node.next;
    }
    while let Some(node) = l2 {
        res.push(node.val);
        l2 = node.next;
    }
    // safe to unwrap since `res` in not empty at this stage
    let mut result = ListNode::new(res.pop().unwrap());
    while let Some(a) = res.pop() {
        let mut r = ListNode::new(a);
        r.next = Some(Box::new(result.clone()));
        result = r;
    }
    Some(Box::new(result))
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_merge_two_lists() {
        assert_eq!(merge_two_lists(None, None), None);
        assert_eq!(
            merge_two_lists(Some(Box::new(ListNode::new(1))), None),
            Some(Box::new(ListNode::new(1)))
        );
        assert_eq!(
            merge_two_lists(None, Some(Box::new(ListNode::new(1)))),
            Some(Box::new(ListNode::new(1)))
        );
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(merge_two_lists(l1, l2), expected);
    }
}

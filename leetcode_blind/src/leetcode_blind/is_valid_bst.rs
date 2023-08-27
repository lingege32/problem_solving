struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::travese_tree(root).0
    }
    pub fn travese_tree(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, Option<i32>, Option<i32>) {
        match root {
            Some(r) => {
                let left = Self::travese_tree(r.borrow().left.clone());
                let right = Self::travese_tree(r.borrow().right.clone());
                if !left.0 || !right.0 {
                    (false, None, None)
                } else {
                    let r_val = r.borrow().val;
                    if let Some(left_max) = left.2 {
                        if left_max >= r_val {
                            return (false, None, None);
                        }
                    }
                    if let Some(right_min) = right.1 {
                        if right_min <= r_val {
                            return (false, None, None);
                        }
                    }
                    (
                        true,
                        Some(left.1.unwrap_or(r_val)),
                        Some(right.2.unwrap_or(r_val)),
                    )
                }
            }
            None => (true, None, None),
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let r1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let r2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let r3 = Rc::new(RefCell::new(TreeNode::new(3)));
        r2.borrow_mut().left = Some(r1);
        r2.borrow_mut().right = Some(r3);
        assert!(Solution::is_valid_bst(Some(r2)));
    }
}

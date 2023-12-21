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

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(r) => {
                Self::is_same_with_symmetric(r.borrow().left.clone(), r.borrow().right.clone())
            }
            None => true,
        }
    }
    fn is_same_with_symmetric(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match left {
            Some(node_l) => match right {
                Some(node_r) => {
                    node_l.borrow().val == node_r.borrow().val
                        && Self::is_same_with_symmetric(
                            node_l.borrow().left.clone(),
                            node_r.borrow().right.clone(),
                        )
                        && Self::is_same_with_symmetric(
                            node_l.borrow().right.clone(),
                            node_r.borrow().left.clone(),
                        )
                }
                None => false,
            },
            None => match right {
                Some(_) => false,
                None => true,
            },
        }
    }
}

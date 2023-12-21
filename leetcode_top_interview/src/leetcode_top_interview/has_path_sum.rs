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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(r) => {
                let v = r.borrow().val;
                if Self::is_leaf(&*r.borrow()) {
                    target_sum == v
                } else {
                    Self::has_path_sum(r.borrow().left.clone(), target_sum - v)
                        || Self::has_path_sum(r.borrow().right.clone(), target_sum - v)
                }
            }
            None => false,
        }
    }
    fn is_leaf(node: &TreeNode) -> bool {
        node.left.is_none() && node.right.is_none()
    }
}

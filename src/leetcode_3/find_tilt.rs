// Definition for a binary tree node.

struct Solution {}
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(r) => {
                let left = Self::find_tilt(r.borrow().left.clone());
                let right = Self::find_tilt(r.borrow().right.clone());
                let left_val = r.borrow().left.as_ref().map(|x| x.borrow().val).unwrap_or(0);
                let right_val = r.borrow().right.as_ref().map(|x| x.borrow().val).unwrap_or(0);
                r.borrow_mut().val += left_val+right_val;
                i32::abs(left_val - right_val) + left + right
            }
            None => 0,
        }
    }
}

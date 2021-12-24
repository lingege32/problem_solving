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
struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn inner(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
            match root {
                Some(r) => {
                    let left = 0.max(inner(r.borrow().left.clone(), max));
                    let right = 0.max(inner(r.borrow().right.clone(), max));
                    *max = (*max).max(left+right+r.borrow().val);
                    left.max(right) + r.borrow().val
                }
                None => 0,
            }
        }
        let mut max = i32::MIN;
        inner(root.clone(), &mut max);
        max
    }
}

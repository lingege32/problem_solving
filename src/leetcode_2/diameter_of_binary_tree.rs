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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        fn max_depth(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
            match root {
                Some(r) => {
                    let left_max = max_depth(r.borrow_mut().left.take(), max);
                    let right_max = max_depth(r.borrow_mut().right.take(), max);
                    *max = (*max).max(left_max + right_max);
                    return left_max.max(right_max) + 1;
                }
                None => {
                    return 0;
                }
            }
        }
        max_depth(root, &mut max);
        max
    }
}

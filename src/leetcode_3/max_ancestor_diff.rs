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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn inner(root: Option<Rc<RefCell<TreeNode>>>, max: Option<i32>, min: Option<i32>) -> i32 {
            match root {
                Some(r) => {
                    let v = r.borrow().val;
                    let diff = i32::abs(max.unwrap_or(v) - v).max(i32::abs(min.unwrap_or(v) - v));
                    let max = max.map(|x| x.max(v)).or(Some(v));
                    let min = min.map(|x| x.min(v)).or(Some(v));
                    let left_diff = inner(r.borrow_mut().left.take(), max, min);
                    let right_diff = inner(r.borrow_mut().right.take(), max, min);
                    diff.max(left_diff).max(right_diff)
                }
                None => 0,
            }
        }
        inner(root, None, None)
    }
}

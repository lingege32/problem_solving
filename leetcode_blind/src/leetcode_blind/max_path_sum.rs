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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map(|v| {
            let mut max_distance = i32::MIN;
            get_max_distance(&v.borrow(), &mut max_distance);
            max_distance
        })
        .unwrap_or(0)
    }
}

fn get_max_distance(node: &TreeNode, max_distance: &mut i32) -> i32 {
    let mut get = |v: &Rc<RefCell<TreeNode>>| get_max_distance(&v.borrow(), max_distance).max(0);
    let to_left = node.left.as_ref().map(&mut get).unwrap_or(0);
    let to_right = node.right.as_ref().map(&mut get).unwrap_or(0);
    *max_distance = (*max_distance).max(node.val + to_left + to_right);
    node.val + to_left.max(to_right)
}

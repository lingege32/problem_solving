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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut inorder = vec![];
        Self::inorder(root, &mut inorder);
        inorder
            .windows(2)
            .map(|v| (v[0] - v[1]).abs())
            .min()
            .unwrap()
    }
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(r) = root {
            Self::inorder(r.borrow().left.clone(), v);
            v.push(r.borrow().val);
            Self::inorder(r.borrow().right.clone(), v);
        }
    }
}

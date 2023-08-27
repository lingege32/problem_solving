struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut v = vec![];
        Self::inner(root, k as usize, &mut v);
        *v.last().unwrap()
    }
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, k: usize, v: &mut Vec<i32>) {
        if v.len() == k {
            return;
        }
        if let Some(root) = root {
            Self::inner(root.borrow().left.clone(), k, v);
            if v.len() == k {
                return;
            }
            let r_val = root.borrow().val;
            v.push(r_val);
            if v.len() == k {
                return;
            }
            Self::inner(root.borrow().right.clone(), k, v);
        }
    }
}

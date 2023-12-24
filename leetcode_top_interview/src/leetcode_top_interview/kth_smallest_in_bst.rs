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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut n = 0;
        Self::inner(root, &mut n, k)
    }
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, num: &mut i32, k: i32) -> i32 {
        match root {
            Some(r) => {
                let v = Self::inner(r.borrow().left.clone(), num, k);
                if v != -1 {
                    return v;
                }
                *num += 1;
                if *num == k {
                    return r.borrow().val;
                }
                Self::inner(r.borrow().right.clone(), num, k)
            }
            None => -1,
        }
    }
}

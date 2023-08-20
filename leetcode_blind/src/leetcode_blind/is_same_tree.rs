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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_some() && q.is_some() {
            let p = p.unwrap();
            let q = q.unwrap();
            if p.borrow().val != q.borrow().val {
                false
            } else {
                let mut p_borrow = p.borrow_mut();
                let mut q_borrow = q.borrow_mut();
                Self::is_same_tree(p_borrow.left.take(), q_borrow.left.take())
                    && Self::is_same_tree(p_borrow.right.take(), q_borrow.right.take())
            }
        } else {
            if p.is_none() && q.is_none() {
                true
            } else {
                false
            }
        }
    }
}

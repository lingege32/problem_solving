// Definition for a binary tree node.
struct Solution;
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
      right: None
    }
  }
}
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    #[allow(dead_code)]
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = &root {
            let left = r.borrow_mut().left.take();
            let right = r.borrow_mut().right.take();
            let left = Self::invert_tree(left);
            let right = Self::invert_tree(right);
            r.borrow_mut().left = right;
            r.borrow_mut().right = left;
        }
        root
    }
}

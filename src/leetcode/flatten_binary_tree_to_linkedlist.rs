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
      right: None
    }
  }
}
struct Solution {
    
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::__flatten(root);
    }
    fn __flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(r) => {
                let mut node_left = r.borrow_mut().left.take();
                let mut node_right = r.borrow_mut().right.take();
                let node_left_last = Self::__flatten(&mut node_left);
                let node_right_last = Self::__flatten(&mut node_right);
                match node_left.as_mut() {
                    Some(_) => {
                        match node_right.as_mut() {
                            Some(_) => {
                                r.borrow_mut().right = node_left;
                                node_left_last.unwrap().borrow_mut().right = node_right;
                                node_right_last
                            },
                            None => {
                                r.borrow_mut().right = node_left;
                                node_left_last
                            }
                        }
                    },
                    None => {
                        match node_right.as_mut() {
                            Some(_) => {
                                r.borrow_mut().right = node_right;
                                node_right_last
                            },
                            None => {
                                root.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}
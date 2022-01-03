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
struct Solution;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inner(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
            if let Some(r) = root {
                inner(r.borrow().left.clone(), ret);
                ret.push(r.borrow().val);
                inner(r.borrow().right.clone(), ret);                
            }
        }
        let mut ret = Vec::new();
        inner(root, &mut ret);
        ret
    }
}
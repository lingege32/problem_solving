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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn inner(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                Some(r) => {
                    let left = inner(r.borrow().left.clone());
                    let right = inner(r.borrow().right.clone());
                    (r.borrow().val + left.1 + right.1, left.0.max(left.1) + right.0.max(right.1))
                },
                None=>(0,0)
            }
        }
        let ans = inner(root);
        ans.0.max(ans.1)
        
    }
}
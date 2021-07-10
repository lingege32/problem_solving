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
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        match root {
            None => vec![],
            Some(r) => {
                let mut ans = vec![];
                let mut queue = VecDeque::with_capacity(1);
                queue.push_back(r);
                while !queue.is_empty(){
                    let mut one_layer = vec![];
                    for _ in 0..queue.len() {
                        let node = queue.pop_front().unwrap();
                        one_layer.push(node.borrow().val);
                        let left = node.borrow().left.clone();
                        if let Some(left) = left {
                            queue.push_back(left.clone());
                        }
                        let right = node.borrow().right.clone();
                        if let Some(right) = right {
                            queue.push_back(right.clone());
                        }
                    }
                    ans.push(one_layer);
                }

                ans
            }
        }
    }
}
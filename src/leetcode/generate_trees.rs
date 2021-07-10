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
use std::rc::Rc;
use std::cell::{Ref, RefCell};
struct Solution {
    
}
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::generate_trees_by_addition(n, 0)
    }
    fn generate_trees_by_addition(n: i32, add: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            vec![None]
        } else {
            let mut res = Vec::new();
            for root in 1..=n {
                let left = Self::generate_trees_by_addition(root-1, add);
                let right = Self::generate_trees_by_addition(n-root, root+add);
                for left_root  in &left {
                    for right_root in &right {
                        let root_node = Rc::new(RefCell::new(TreeNode::new(root+add)));
                        root_node.borrow_mut().left = left_root.clone();
                        root_node.borrow_mut().right = right_root.clone();
                        res.push(Some(root_node));
                    }
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let a = Solution::generate_trees(3);
        for i in a {
            println!("{:?}", i);
        }
        assert!(false);
    }
}
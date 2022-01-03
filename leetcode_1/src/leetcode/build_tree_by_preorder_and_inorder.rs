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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_index: std::collections::HashMap<_,_> = inorder.into_iter().enumerate().map(|(i,v)| (v,i as i32)).collect();

        build(&preorder, &inorder_index, &mut 0, 0, preorder.len() as i32 -1)
    }
}

fn build(preorder: &Vec<i32>, inorder_index: &std::collections::HashMap<i32,i32>, i: &mut usize, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if left > right {
        return None;
    }

    let val = preorder[*i];
    let mut root = TreeNode::new(val);
    *i += 1;

    root.left = build(preorder, inorder_index, i, left, inorder_index[&val]-1);
    root.right = build(preorder, inorder_index, i, inorder_index[&val]+1, right);
    return Some(Rc::new(RefCell::new(root)));
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let a = Solution::build_tree([1,2].to_vec(), [2,1].to_vec());
        assert!(true);
    }
}

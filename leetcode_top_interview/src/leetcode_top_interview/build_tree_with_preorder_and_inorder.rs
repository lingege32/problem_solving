// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution;
impl TreeNode {
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::inner(&preorder, &inorder)
    }
    fn inner(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let root_val = preorder[0];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        if preorder.len() != 1 {
            let position = inorder.iter().position(|&x| x == root_val).unwrap();
            let left = Self::inner(&preorder[1..1 + position], &inorder[..position]);
            let right = Self::inner(&preorder[1 + position..], &inorder[position + 1..]);
            root.borrow_mut().left = left;
            root.borrow_mut().right = right;
        }
        Some(root)
    }
}

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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    #[allow(dead_code)]
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Node {
        Self::inner(&inorder, &postorder)
    }
    fn inner(inorder: &[i32], postorder: &[i32]) -> Node {
        if inorder.is_empty() {
            return None;
        }

        let root_val = *postorder.last().unwrap();
        let position = inorder.iter().position(|x| *x == root_val).unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Self::inner(&inorder[..position], &postorder[..position]),
            right: Self::inner(
                &inorder[position + 1..],
                &postorder[position..postorder.len() - 1],
            ),
        })))
    }
}

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
            right: None,
        }
    }
}
struct Solution {}
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder = inorder
            .into_iter()
            .enumerate()
            .map(|(idx, val)| (val, idx as i32))
            .collect();
        Self::build(
            &postorder,
            &mut (postorder.len() - 1),
            0,
            postorder.len() as i32 - 1,
            &inorder,
        )
    }
    fn build(
        postorder: &[i32],
        i: &mut usize,
        left: i32,
        right: i32,
        inorder: &HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if right < left {
            None
        } else {
            let rv = postorder[*i];
            *i -= 1;
            let mid = inorder[&rv];
            let mut root = TreeNode {
                val: rv,
                right: Self::build(postorder, i, mid + 1, right, inorder),
                left: Self::build(postorder, i, left, mid - 1, inorder),
            };
            Some(Rc::new(RefCell::new(root)))
        }
    }
}

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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder, &inorder)
    }
    fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            None
        } else {
            // println!("pre: {:?}, ino: {:?}", preorder, inorder);
            let m = preorder[0];
            let i = inorder.iter().position(|&x| x == m).unwrap();
            let (left, right) = inorder.split_at(i);
            let left_len = left.len();
            let left_node = Self::build(&preorder[1..1 + left_len], left);
            let right_node = Self::build(&preorder[1 + left_len..], &right[1..]);
            Some(Rc::new(RefCell::new(TreeNode {
                val: m,
                left: left_node,
                right: right_node,
            })))
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let pre = vec![3, 9, 20, 15, 7];
        let ino = vec![9, 3, 15, 20, 7];
        Solution::build_tree(pre, ino);
    }
}

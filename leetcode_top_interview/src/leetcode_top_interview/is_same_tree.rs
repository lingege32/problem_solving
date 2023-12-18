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
impl Solution {
    #[allow(dead_code)]
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match p {
            Some(p_n) => match q {
                Some(q_n) => {
                    let mut p_b = p_n.borrow_mut();
                    let mut q_b = q_n.borrow_mut();
                    p_b.val == q_b.val
                        && Self::is_same_tree(p_b.left.take(), q_b.left.take())
                        && Self::is_same_tree(p_b.right.take(), q_b.right.take())
                }
                None => false,
            },
            None => match q {
                Some(_) => false,
                None => true,
            },
        }
    }
}

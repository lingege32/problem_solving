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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let Some(r) = root else { return };
        let mut left = r.borrow_mut().left.take();
        let mut right = r.borrow_mut().right.take();
        Self::flatten(&mut left);
        r.borrow_mut().right = left;
        let mut node = Some(r.clone());
        while node.is_some() {
            if node.as_ref().unwrap().borrow().right.is_none() {
                break;
            }
            node = node.unwrap().borrow().right.clone();
        }

        Self::flatten(&mut right);
        node.as_mut().unwrap().borrow_mut().right = right;
    }
}

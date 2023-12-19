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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = vec![];
        Self::inner(root, &mut ret, 0);
        ret
    }
    fn inner(node: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>, level: usize) {
        if let Some(n) = node {
            if level >= v.len() {
                v.resize(level + 1, 0);
            }
            v[level] = n.borrow().val;
            Self::inner(n.borrow().left.clone(), v, level + 1);
            Self::inner(n.borrow().right.clone(), v, level + 1);
        }
    }
}

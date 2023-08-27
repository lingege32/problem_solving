struct Solution;
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        let (min, max) = if p < q { (p, q) } else { (q, p) };
        Self::inner(root, min, max)
    }

    pub fn inner(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(r) => {
                let v = r.borrow().val;
                if v == p || v == q {
                    return Some(r);
                }
                let left = r.borrow().left.clone();
                let right = r.borrow().right.clone();

                if v < p {
                    Self::inner(right, p, q)
                } else if v > q {
                    Self::inner(left, p, q)
                } else {
                    Some(r)
                }
            }
        }
    }
}

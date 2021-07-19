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
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = Self::inner(root, target_sum);
        for v in res.iter_mut() {
            v.reverse();
        }
        res
    }
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if let Some(r) = root {
            fn is_leaf(node: &Rc<RefCell<TreeNode>>) -> bool {
                node.borrow().left.is_none() && node.borrow().right.is_none()
            }
            if is_leaf(&r) {
                if r.borrow().val == target_sum {
                    vec![vec![target_sum]]
                } else {
                    Vec::new()
                }
            } else {
                let val = r.borrow().val;
                let mut left_v = Self::inner(r.borrow().left.clone(), target_sum - val);
                let mut right_v = Self::inner(r.borrow().right.clone(), target_sum - val);
                left_v.append(&mut right_v);
                for v in left_v.iter_mut() {
                    v.push(val);
                }
                left_v
            }
        } else {
            Vec::new()
        }
    }
}

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut arr1 = vec![root];
        while !arr1.is_empty() {
            let mut v = vec![];
            let mut arr2 = vec![];
            for r in arr1.iter() {
                if let Some(rr) = r {
                    v.push(rr.borrow().val);
                    arr2.push(rr.borrow().left.clone());
                    arr2.push(rr.borrow().right.clone());
                }
            }
            if arr2.is_empty() {
                break;
            }
            ret.push(v);
            unsafe {
                std::ptr::swap(&mut arr1, &mut arr2);
            }
        }

        ret
    }
}

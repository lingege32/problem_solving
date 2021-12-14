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
    #[allow(dead_code)]
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                let r = Some(r);
                let mut stack = Vec::new();
                stack.push(r);
                let mut ans = 0;
                while let Some(p) = stack.pop() {
                    match p {
                        None => continue,
                        Some(r) => {
                            let rr = r;
                            let v = rr.borrow().val;
                            if v >= low && v <= high {
                                if v >= low {
                                    ans += v;
                                }
                            }

                            stack.push(rr.borrow_mut().right.take());
                            stack.push(rr.borrow_mut().left.take());
                        }
                    }
                }
                ans
            }
        }
    }
}

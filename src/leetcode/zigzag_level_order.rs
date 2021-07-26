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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(r) = root {
            let mut stack = VecDeque::new();
            stack.push_back(r.clone());
            let mut ans = Vec::new();
            while !stack.is_empty() {
                let mut alevel = Vec::new();
                let l = stack.len();
                for _ in 0..l {
                    let node = stack.pop_front().unwrap();
                    alevel.push(node.borrow().val);
                    let b = node.borrow();
                    if let Some(le) = b.left.as_ref() {
                        stack.push_back(le.clone());
                    }
                    if let Some(ri) = b.right.as_ref() {
                        stack.push_back(ri.clone());
                    }
                }
                ans.push(alevel);
            }
            for alevel in ans[1..].iter_mut().step_by(2) {
                alevel.reverse();
            }
            ans
        } else {
            Vec::new()
        }
    }
}

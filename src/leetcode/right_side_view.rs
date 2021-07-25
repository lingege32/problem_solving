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
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut view = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r.clone());
        }
        while !queue.is_empty() {
            let level = queue.len();
            view.push(queue.front().unwrap().borrow().val);
            for _ in 0..level {
                let n = queue.pop_front().unwrap();
                if let Some(r) = &n.borrow().right {
                    queue.push_back(r.clone());
                };
                if let Some(l) = &n.borrow().left {
                    queue.push_back(l.clone());
                };
            }
        }
        view
    }
}

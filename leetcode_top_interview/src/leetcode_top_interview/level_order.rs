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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        queue.push_back(root);
        let mut ret = vec![];
        loop {
            let len = queue.len();
            let mut level = vec![];
            for _ in 0..len {
                let node = queue.pop_front().unwrap();
                if let Some(n) = node {
                    level.push(n.borrow().val);
                    queue.push_back(n.borrow_mut().left.take());
                    queue.push_back(n.borrow_mut().right.take());
                }
            }
            if level.is_empty() {
                break;
            } else {
                ret.push(level);
            }
        }
        ret
    }
}

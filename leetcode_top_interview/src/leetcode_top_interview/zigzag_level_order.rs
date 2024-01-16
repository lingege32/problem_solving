// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut queue = vec![root.unwrap()];
        let mut ret = vec![];
        let mut is_forward = true;
        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            ret.push({
                let i = queue.iter().map(|x| x.borrow().val);
                if is_forward {
                    i.collect::<Vec<_>>()
                } else {
                    i.rev().collect::<Vec<_>>()
                }
            });
            for node in queue {
                if let Some(l) = node.borrow().left.clone() {
                    next_queue.push(l);
                }
                if let Some(r) = node.borrow().right.clone() {
                    next_queue.push(r);
                }
            }
            is_forward = !is_forward;
            queue = next_queue;
        }
        ret
    }
}

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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        if root.is_none() {
            return vec![];
        }
        let mut ret = vec![];

        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let num = queue.len();
            let total = (0..num)
                .map(|_| {
                    let node = queue.pop_front().unwrap();
                    if let Some(n) = node {
                        let l = n.borrow().left.clone();
                        let r = n.borrow().right.clone();
                        if l.is_some() {
                            queue.push_back(l)
                        }
                        if r.is_some() {
                            queue.push_back(r)
                        }
                        n.borrow().val as i64
                    } else {
                        0i64
                    }
                })
                .sum::<i64>();
            ret.push(total as f64 / num as f64);
        }

        ret
    }
}

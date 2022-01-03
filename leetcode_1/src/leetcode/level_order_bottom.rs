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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let l = queue.len();
            let mut v = Vec::new();
            for _ in 0..l {
                let n = queue.pop_front().unwrap();
                if let Some(node) = n {
                    v.push(node.borrow().val);
                    queue.push_back(node.borrow().left.clone());
                    queue.push_back(node.borrow().right.clone());
                }
            }
            if !v.is_empty() {
                ans.push(v);
            }
        }
        ans.reverse();
        ans
    }
}

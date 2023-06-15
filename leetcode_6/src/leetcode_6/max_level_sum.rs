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
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cur = vec![root];
        let mut next = vec![];
        let mut max_level = 0;
        let mut cur_max = i32::MIN;
        let mut level = 0;
        while !cur.is_empty() {
            level += 1;
            let sum = cur
                .into_iter()
                .map(|y| {
                    let unwrap = y.unwrap();
                    let mut m = unwrap.borrow_mut();
                    let l = m.left.take();
                    let r = m.right.take();
                    if l.is_some() {
                        next.push(l);
                    }
                    if r.is_some() {
                        next.push(r);
                    }
                    m.val
                })
                .sum::<i32>();
            if sum > cur_max {
                cur_max = sum;
                max_level = level;
            }
            cur = next;
            next = vec![];
        }
        max_level
    }
}

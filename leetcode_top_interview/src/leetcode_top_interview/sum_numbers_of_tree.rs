// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::inner(&root, 0, &mut ans);
        ans
    }
    fn inner(root: &Option<Rc<RefCell<TreeNode>>>, mut cur: i32, ans: &mut i32) {
        if let Some(h) = root {
            cur *= 10;
            cur += h.borrow().val;
            if h.borrow().left.is_none() && h.borrow().right.is_none() {
                *ans += cur;
            } else {
                Self::inner(&h.borrow().left, cur, ans);
                Self::inner(&h.borrow().right, cur, ans);
            }
        }
    }
}

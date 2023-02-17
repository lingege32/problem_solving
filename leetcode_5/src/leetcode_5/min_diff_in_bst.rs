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
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = i32::MAX;
        let mut stack = vec![];
        Self::helper(&root, &mut stack, &mut ans);
        ans
    }
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>, ans: &mut i32) {
        if *ans == 1 {
            return;
        }
        if let Some(r) = root {
            let v = r.borrow().val;
            if !stack.is_empty() {
                let dis = stack.iter().map(|x| (v-x).abs()).min().unwrap();
                *ans = dis.min(*ans);
            }
            stack.push(v);
            Self::helper(&r.borrow().left, stack, ans);
            Self::helper(&r.borrow().right, stack, ans);
            stack.pop();
        }
    }
}

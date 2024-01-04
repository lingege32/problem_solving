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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = i32::MIN;
        Self::inner(root, &mut max);
        max
    }

    fn inner(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        match root {
            Some(r) => {
                let l_v = 0.max(Self::inner(r.borrow().left.clone(), max));
                let r_v = 0.max(Self::inner(r.borrow().right.clone(), max));
                let val = r.borrow().val;
                let t = val+l_v+r_v;
                *max = t.max(*max);
                l_v.max(r_v) + val
            }
            None => 0,
        }
    }
}

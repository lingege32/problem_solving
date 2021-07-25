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
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = None;
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: *mut TreeNode,
            q: *mut TreeNode,
            ans: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if ans.is_some() {
                return false;
            }
            match root {
                None => false,
                Some(rn) => {
                    let mid = if rn.as_ptr() == p || rn.as_ptr() == q {
                        1
                    } else {
                        0
                    };
                    let l = if dfs(rn.borrow().left.clone(), p, q, ans) {
                        1
                    } else {
                        0
                    };
                    let r = if dfs(rn.borrow().right.clone(), p, q, ans) {
                        1
                    } else {
                        0
                    };
                    if l + r + mid == 2 {
                        *ans = Some(rn.clone());
                        true
                    } else {
                        l + r + mid > 0
                    }
                }
            }
        }
        let p = p.unwrap().as_ptr();
        let q = q.unwrap().as_ptr();
        dfs(root, p, q, &mut ans);
        ans
    }
}

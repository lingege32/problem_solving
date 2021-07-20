
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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> i32 {
            fn is_leaf(r: &Rc<RefCell<TreeNode>>) -> bool {
                r.borrow().left.is_none() && r.borrow().right.is_none()
            }
            if let Some(r) = root {
                if is_leaf(&r) {
                    *k -= 1;
                    if *k==0 {
                        r.borrow().val
                    } else {
                        0
                    }
                } else {
                    let v = dfs(r.borrow().left.clone(), k);
                    if *k == 0 {
                        return v;
                    }
                    *k -= 1 ;
                    if *k==0 {
                        r.borrow().val
                    } else {
                        dfs(r.borrow().right.clone(), k)
                    }
                }
            } else {
                0
            }
        }
        let mut k = k;
        dfs(root, &mut k)
    }
}

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

use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let mut res = 0;
            fn is_leaf(node: &Rc<RefCell<TreeNode>>) -> bool {
                node.borrow().left.is_none() && node.borrow().right.is_none()
            }
            fn summary(v: &[i32]) -> i32 {
                let mut unit = 1;
                v.iter().rev().fold(0, move |acc, &x| {
                    let r = acc + x*unit;
                    unit*=10;
                    r
                })
            }
            
            fn dfs(root: Rc<RefCell<TreeNode>>, res: &mut i32, visit_stack: &mut Vec<i32>) {
                visit_stack.push(root.borrow().val);
                if is_leaf(&root) {
                    *res+=summary(&visit_stack);
                } else {
                    if let Some(l) = root.borrow().left.as_ref() {
                        dfs(l.clone(), res, visit_stack);
                    }
                    if let Some(r) = root.borrow().right.as_ref() {
                        dfs(r.clone(), res, visit_stack);
                    }
                }
                visit_stack.pop();
            }
            let mut visit_stack = Vec::new();
            let mut res = 0;
            dfs(r, &mut res, &mut visit_stack);
            res

        } else {
            0
        }
    }
}

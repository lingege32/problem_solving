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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut stack = vec![root.clone()];
        while !stack.is_empty() {
            let new_stack = stack
                .iter()
                .filter(|x| x.is_some())
                .flat_map(|x| {
                    let xx = x.as_ref().unwrap().borrow();
                    vec![xx.left.clone(), xx.right.clone()]
                })
                .collect();

            let mut sub = vec![];
            if ans.len() % 2 == 0 {
                for node in stack {
                    if let Some(n) = node {
                        sub.push(n.borrow().val);
                    }
                }
            } else {
                for node in stack.into_iter().rev() {
                    if let Some(n) = node {
                        sub.push(n.borrow().val);
                    }
                }
            }
            ans.push(sub);
            stack = new_stack;
        }
        ans.resize(ans.len() - 1, Default::default());
        ans
    }
}

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(r) => {
                type Node = Option<Rc<RefCell<TreeNode>>>;
                fn is_same(left: &Node, right: &Node) -> bool {
                    match left {
                        Some(l) => match right {
                            Some(r) => {
                                l.borrow().val == r.borrow().val
                                    && is_same(&l.borrow().left, &r.borrow().right)
                                    && is_same(&l.borrow().right, &r.borrow().left)
                            }
                            None => false,
                        },
                        None => right.is_none(),
                    }
                }
                is_same(&r.borrow().left, &r.borrow().right)
            }
            None => true,
        }
    }
}

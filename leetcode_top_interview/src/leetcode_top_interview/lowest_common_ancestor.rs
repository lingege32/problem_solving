// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        Self::inner(root, p, q)
    }
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                if val == p || val == q {
                    return Some(node);
                }

                let l = Self::inner(node.borrow().left.clone(), p, q);
                let r = Self::inner(node.borrow().right.clone(), p, q);
                if l.is_some() && r.is_some() {
                    Some(node)
                } else if l.is_some() {
                    l
                } else if r.is_some() {
                    r
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

struct Solution;
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if Self::is_same(&root, &sub_root) {
            true
        } else {
            match root {
                Some(r) => {
                    let mut rm = r.borrow_mut();
                    Self::is_subtree(rm.left.take(), sub_root.clone())
                        || Self::is_subtree(rm.right.take(), sub_root)
                }
                None => false,
            }
        }
    }
    fn is_same(lhs: &Option<Rc<RefCell<TreeNode>>>, rhs: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if lhs.is_none() && rhs.is_none() {
            true
        } else if lhs.is_some() && rhs.is_some() {
            let l = lhs.as_ref().unwrap();
            let r = rhs.as_ref().unwrap();
            let lb = l.borrow();
            let rb = r.borrow();
            lb.val == rb.val
                && Self::is_same(&lb.left, &rb.left)
                && Self::is_same(&lb.right, &rb.right)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let n4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let n5 = Rc::new(RefCell::new(TreeNode::new(5)));
        n4.borrow_mut().left = Some(n1);
        n4.borrow_mut().right = Some(n2);
        n3.borrow_mut().left = Some(n4);
        n3.borrow_mut().right = Some(n5);

        let s1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let s2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let s4 = Rc::new(RefCell::new(TreeNode::new(4)));
        s4.borrow_mut().left = Some(s1);
        s4.borrow_mut().right = Some(s2);

        assert_eq!(true, Solution::is_subtree(Some(n3), Some(s4)));
    }
}

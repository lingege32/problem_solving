// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = Self::list_len(head.as_ref());
        Self::build(head, len).0
    }
    fn list_len(mut head: Option<&Box<ListNode>>) -> usize {
        let mut len = 0;
        while let Some(h) = head {
            len += 1;
            head = h.next.as_ref();
        }
        len
    }
    fn build(
        head: Option<Box<ListNode>>,
        len: usize,
    ) -> (Option<Rc<RefCell<TreeNode>>>, Option<Box<ListNode>>) {
        if len == 0 {
            (None, head)
        } else {
            let mid = len / 2;
            let (left, head) = Self::build(head, mid);
            let mut root = TreeNode::new(head.as_ref().unwrap().val);
            let (right, head) = Self::build(head.unwrap().next, len - mid - 1);
            root.left = left;
            root.right = right;
            (Some(Rc::new(RefCell::new(root))), head)
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = ListNode::new(1);
        let mut b = ListNode::new(2);
        let mut c = ListNode::new(3);
        let mut d = ListNode::new(4);
        let mut e = ListNode::new(5);
        d.next = Some(Box::new(e));
        c.next = Some(Box::new(d));
        b.next = Some(Box::new(c));
        a.next = Some(Box::new(b));
        let t = Solution::sorted_list_to_bst(Some(Box::new(a)));
        println!("t: {:#?}",t);
        assert!(true);
    }
}